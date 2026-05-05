/// Text-level include preprocessor for PHPRS.
/// Resolves `include "path/file.phprs";` and `require "path/file.phprs";` directives
/// before lexing. Also handles `include_once` / `require_once` with duplicate prevention.
/// Syntax: include "relative/path.phprs";
///         require "relative/path.phprs";
///         include_once "relative/path.phprs";
///         require_once "relative/path.phprs";
/// The directive must appear at the start of a line (leading whitespace allowed).
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Preprocess a PHPRS source string by resolving all `include "..."` directives.
/// `current_dir` is used to resolve relative paths.
pub fn preprocess(source: &str, current_dir: &Path) -> Result<String, String> {
    let mut included: HashSet<PathBuf> = HashSet::new();
    preprocess_inner(source, current_dir, &mut included)
}

fn preprocess_inner(
    source: &str,
    current_dir: &Path,
    included: &mut HashSet<PathBuf>,
) -> Result<String, String> {
    let mut output = String::new();
    let mut in_phprs_tag = false;

    for raw_line in source.lines() {
        let line = raw_line;
        let trimmed = line.trim();

        // Track <?phprs / ?> tag state
        if trimmed.contains("<?phprs") {
            in_phprs_tag = true;
        }

        // Check for include / require / include_once / require_once directive
        let inc_directive = parse_include_directive(trimmed);
        if let Some((path_str, is_once, is_require)) = inc_directive {
            let include_path = match resolve_include_path(path_str, current_dir) {
                Ok(p) => p,
                Err(e) => {
                    if is_require {
                        return Err(e);
                    }
                    // include / include_once: warning and skip
                    eprintln!("Warning: {}", e);
                    output.push_str(line);
                    output.push('\n');
                    continue;
                }
            };

            // Canonicalize to prevent duplicates
            let canonical = include_path.canonicalize().unwrap_or(include_path.clone());

            if is_once && included.contains(&canonical) {
                continue; // skip duplicate include_once / require_once
            }
            if is_once {
                included.insert(canonical.clone());
            }

            let content = read_include_file(&include_path)?;
            let parent_dir = include_path.parent().unwrap_or(current_dir);

            // Recursively preprocess included file
            let processed = preprocess_inner(&content, parent_dir, included)?;

            // Handle tag stripping for included files:
            // If the current file is inside a PHPRS tag, strip <?phprs / ?> from included file
            let processed = if in_phprs_tag {
                strip_phprs_tags(&processed)
            } else {
                processed
            };

            output.push_str(&processed);
            output.push('\n');
        } else {
            output.push_str(line);
            output.push('\n');
        }

        if trimmed.contains("?>") {
            in_phprs_tag = false;
        }
    }

    Ok(output)
}

/// Parse an include/require/include_once/require_once directive.
/// Returns Some((path, is_once)) or None if the line is not an include directive.
/// Parse an include/require/include_once/require_once directive.
/// Returns Some((path, is_once, is_require)) or None if the line is not an include directive.
fn parse_include_directive(line: &str) -> Option<(&str, bool, bool)> {
    let directives = [
        ("include_once \"", true, false),
        ("require_once \"", true, true),
        ("include \"", false, false),
        ("require \"", false, true),
    ];

    for (prefix, is_once, is_require) in &directives {
        if line.starts_with(prefix) && line.ends_with("\";") {
            let path = &line[prefix.len()..line.len() - 2];
            return Some((path, *is_once, *is_require));
        }
    }
    None
}

fn resolve_include_path(path_str: &str, current_dir: &Path) -> Result<PathBuf, String> {
    let path = Path::new(path_str);
    if path.is_absolute() {
        if path.exists() {
            Ok(path.to_path_buf())
        } else {
            Err(format!("Include file not found: {}", path_str))
        }
    } else {
        let full = current_dir.join(path);
        if full.exists() {
            Ok(full)
        } else {
            Err(format!("Include file not found: '{}' (looked in '{}')",
                path_str, current_dir.display()))
        }
    }
}

fn read_include_file(path: &Path) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("Cannot read include file '{}': {}", path.display(), e))
}

/// Strip <?phprs and ?> tags from preprocessed content.
/// When a file is included inside a PHPRS block, the tags from the included
/// file should be removed to avoid nested tag issues.
fn strip_phprs_tags(content: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;

    for ch in content.chars() {
        if !in_tag {
            result.push(ch);
            if result.ends_with("<?phprs") {
                // Remove the just-added "<?phprs" prefix
                result.truncate(result.len() - 7);
                in_tag = true;
            }
        } else {
            // Inside a PHPRS tag — look for ?>
            result.push(ch);
            if result.ends_with("?>") {
                // Remove the "?>" suffix
                result.truncate(result.len() - 2);
                in_tag = false;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_basic_include() {
        let tmp = env::temp_dir().join("phprs_preprocessor_test");
        let _ = fs::create_dir_all(&tmp);

        let main = format!("<?phprs\necho \"Main\";\ninclude \"{}/mod.phprs\";\necho \"End\";\n?>",
            tmp.display().to_string().replace('\\', "/"));

        let mod_content = "<?phprs\necho \"Module\";\n?>";
        let mod_path = tmp.join("mod.phprs");
        let mut f = fs::File::create(&mod_path).unwrap();
        f.write_all(mod_content.as_bytes()).unwrap();

        let result = preprocess(&main, &tmp).unwrap();
        assert!(result.contains("echo \"Main\""));
        assert!(result.contains("echo \"Module\""));
        assert!(result.contains("echo \"End\""));
        // Tags from included file should be stripped
        assert!(!result.contains("<?phprs\necho \"Module\";\n?>"));

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_duplicate_prevention() {
        let tmp = env::temp_dir().join("phprs_pp_dup_test");
        let _ = fs::create_dir_all(&tmp);

        let mod_content = "<?phprs\necho \"Once\";\n?>";
        let mod_path = tmp.join("shared.phprs");
        fs::write(&mod_path, mod_content).unwrap();
        let path_str = mod_path.display().to_string().replace('\\', "/");

        let main = format!("<?phprs\ninclude_once \"{}\";\ninclude_once \"{}\";\n?>", path_str, path_str);

        let result = preprocess(&main, &tmp).unwrap();
        // "Once" should appear only once
        assert_eq!(result.matches("Once").count(), 1);

        let _ = fs::remove_dir_all(&tmp);
    }
}
