//! Custom markdown processing with AST to HTML conversion
//!
//! This module provides markdown parsing using markdown-rs and converts the AST
//! to HTML with custom class support and extensions.

use markdown::{mdast, to_mdast, ParseOptions};

/// Configuration for markdown parsing and HTML generation
#[derive(Debug, Clone)]
pub struct MarkdownConfig {
    /// CSS classes for different element types
    pub heading_class: Option<String>,
    pub paragraph_class: Option<String>,
    pub italic_class: Option<String>,
    pub strong_class: Option<String>,
    pub link_class: Option<String>,
    pub blockquote_class: Option<String>,
    pub code_class: Option<String>,
    pub pre_class: Option<String>,
    pub ul_class: Option<String>,
    pub ol_class: Option<String>,
    pub li_class: Option<String>,
    pub table_class: Option<String>,
    pub th_class: Option<String>,
    pub td_class: Option<String>,
    pub hr_class: Option<String>,
    pub quote_class: Option<String>,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            heading_class: None,
            paragraph_class: None,
            italic_class: None,
            strong_class: None,
            link_class: None,
            blockquote_class: None,
            code_class: None,
            pre_class: None,
            ul_class: None,
            ol_class: None,
            li_class: None,
            table_class: None,
            th_class: None,
            td_class: None,
            hr_class: None,
            quote_class: Some("text-orange-500".to_string()),
        }
    }
}

/// Parse markdown to AST and convert to HTML with custom classes
pub fn markdown_to_html(content: &str, config: &MarkdownConfig) -> Result<String, String> {
    // First process custom quote syntax before parsing
    let processed_content = process_custom_quotes(content, config);
    
    // Configure parse options with extensions
    let mut options = ParseOptions::default();
    options.constructs.gfm_table = true;
    options.constructs.gfm_strikethrough = true;
    options.constructs.gfm_task_list_item = true;
    options.constructs.frontmatter = true;
    
    // Parse to AST
    let ast = to_mdast(&processed_content, &options).map_err(|e| format!("Parse error: {:?}", e))?;
    
    // Convert AST to HTML
    let html = ast_to_html(&ast, config);
    
    // Post-process HTML to handle our custom quotes
    let final_html = post_process_quotes(&html, config);
    
    Ok(final_html)
}

/// Process custom quote syntax: "text" -> custom span elements
/// 
/// This function parses custom quotes while respecting markdown code blocks and inline code.
/// It converts "text" into <quote>"text"</quote> markup that will be handled in the AST.
fn process_custom_quotes(content: &str, _config: &MarkdownConfig) -> String {
    let mut result = String::with_capacity(content.len() * 2); // Pre-allocate more space for markup
    let mut chars = content.chars().peekable();
    let mut in_code_block = false;
    let mut in_inline_code = false;
    let mut code_block_fence_count = 0;
    
    while let Some(ch) = chars.next() {
        match ch {
            // Track code blocks and inline code to avoid processing quotes inside them
            '`' => {
                let _start_pos = result.len();
                result.push(ch);
                
                // Count consecutive backticks
                let mut tick_count = 1;
                while chars.peek() == Some(&'`') {
                    tick_count += 1;
                    result.push(chars.next().unwrap());
                }
                
                // Handle code block fences (3+ backticks)
                if tick_count >= 3 {
                    if !in_code_block {
                        // Starting a code block
                        in_code_block = true;
                        code_block_fence_count = tick_count;
                    } else if tick_count >= code_block_fence_count {
                        // Ending a code block (needs at least as many backticks as the opening)
                        in_code_block = false;
                        code_block_fence_count = 0;
                    }
                } else if tick_count == 1 && !in_code_block {
                    // Toggle inline code (single backtick, not in code block)
                    in_inline_code = !in_inline_code;
                }
            }
            
            // Process quotes only if not in code blocks or inline code
            '"' if !in_code_block && !in_inline_code => {
                if let Some(quote_result) = try_parse_quote(&mut chars) {
                    // Successfully parsed a quote
                    result.push_str(&format!("<hearth-quote>{}</hearth-quote>", quote_result));
                } else {
                    // Not a valid quote, just add the quote character
                    result.push('"');
                }
            }
            
            // Handle line breaks that might end code blocks
            '\n' => {
                result.push(ch);
                // Check if we're in a code block and this might be the end
                if in_code_block {
                    // Look ahead to see if the next line starts with the closing fence
                    let mut peek_chars = chars.clone();
                    let _line_start = true;
                    let mut spaces = 0;
                    
                    // Skip leading whitespace
                    while let Some(&next_ch) = peek_chars.peek() {
                        if next_ch == ' ' || next_ch == '\t' {
                            spaces += 1;
                            peek_chars.next();
                        } else {
                            break;
                        }
                    }
                    
                    // Check for closing fence
                    if spaces <= 3 { // Code block fences can have up to 3 spaces of indentation
                        let mut fence_ticks = 0;
                        while let Some(&next_ch) = peek_chars.peek() {
                            if next_ch == '`' {
                                fence_ticks += 1;
                                peek_chars.next();
                            } else {
                                break;
                            }
                        }
                        
                        if fence_ticks >= code_block_fence_count {
                            // This looks like a closing fence, we'll let the ` handler deal with it
                        }
                    }
                }
            }
            
            // Pass through all other characters
            _ => {
                result.push(ch);
            }
        }
    }
    
    result
}

/// Try to parse a quote starting from the current position
/// Returns Some(quote_content) if a valid quote is found, None otherwise
fn try_parse_quote(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
    let mut quote_content = String::new();
    let mut found_closing = false;
    let mut escaped = false;
    
    // Look for closing quote, handling escapes
    while let Some(ch) = chars.next() {
        match ch {
            '\\' if !escaped => {
                // Start of escape sequence
                escaped = true;
                quote_content.push(ch);
            }
            '"' if !escaped => {
                // Found closing quote
                found_closing = true;
                break;
            }
            '\n' | '\r' => {
                // Quotes don't span lines in our implementation
                // Put the content back and return None
                break;
            }
            _ => {
                escaped = false;
                quote_content.push(ch);
            }
        }
    }
    
    // Only return the quote if we found a proper closing quote and have content
    if found_closing && !quote_content.trim().is_empty() {
        Some(format!("\"{}\"", quote_content))
    } else {
        None
    }
}

/// Convert markdown AST to HTML with custom classes
fn ast_to_html(node: &mdast::Node, config: &MarkdownConfig) -> String {
    match node {
        mdast::Node::Root(root) => {
            root.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("")
        }
        
        mdast::Node::Paragraph(paragraph) => {
            let content = paragraph.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            let class_attr = config.paragraph_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            format!("<p{}>{}</p>", class_attr, content)
        }
        
        mdast::Node::Heading(heading) => {
            let content = heading.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            let class_attr = config.heading_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            format!("<h{}{}>{}</h{}>", heading.depth, class_attr, content, heading.depth)
        }
        
        mdast::Node::Text(text) => {
            html_escape(&text.value)
        }
        
        mdast::Node::Emphasis(emphasis) => {
            let content = emphasis.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            let class_attr = config.italic_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            format!("<em{}>{}</em>", class_attr, content)
        }
        
        mdast::Node::Strong(strong) => {
            let content = strong.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            let class_attr = config.strong_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            format!("<strong{}>{}</strong>", class_attr, content)
        }
        
        mdast::Node::Link(link) => {
            let content = link.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            let class_attr = config.link_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            let title_attr = link.title.as_ref()
                .map(|t| format!(" title=\"{}\"", html_escape(t)))
                .unwrap_or_default();
            
            format!("<a href=\"{}\"{}{}>{}</a>", html_escape(&link.url), class_attr, title_attr, content)
        }
        
        mdast::Node::Blockquote(blockquote) => {
            let content = blockquote.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            let class_attr = config.blockquote_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            format!("<blockquote{}>{}</blockquote>", class_attr, content)
        }
        
        mdast::Node::InlineCode(code) => {
            let class_attr = config.code_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            format!("<code{}>{}</code>", class_attr, html_escape(&code.value))
        }
        
        mdast::Node::Code(code) => {
            let pre_class_attr = config.pre_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            let code_class_attr = config.code_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            let lang_attr = code.lang.as_ref()
                .map(|l| format!(" data-lang=\"{}\"", html_escape(l)))
                .unwrap_or_default();
            
            format!("<pre{}><code{}{}>{}</code></pre>", 
                   pre_class_attr, code_class_attr, lang_attr, html_escape(&code.value))
        }
        
        mdast::Node::List(list) => {
            let content = list.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            if list.ordered {
                let class_attr = config.ol_class.as_ref()
                    .map(|c| format!(" class=\"{}\"", c))
                    .unwrap_or_default();
                
                let start_attr = if let Some(start) = list.start {
                    if start != 1 {
                        format!(" start=\"{}\"", start)
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                
                format!("<ol{}{}>{}</ol>", class_attr, start_attr, content)
            } else {
                let class_attr = config.ul_class.as_ref()
                    .map(|c| format!(" class=\"{}\"", c))
                    .unwrap_or_default();
                
                format!("<ul{}>{}</ul>", class_attr, content)
            }
        }
        
        mdast::Node::ListItem(item) => {
            let content = item.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            let class_attr = config.li_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            format!("<li{}>{}</li>", class_attr, content)
        }
        
        mdast::Node::Table(table) => {
            let content = table.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            let class_attr = config.table_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            format!("<table{}>{}</table>", class_attr, content)
        }
        
        mdast::Node::TableRow(row) => {
            let content = row.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            format!("<tr>{}</tr>", content)
        }
        
        mdast::Node::TableCell(cell) => {
            let content = cell.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            // Determine if this is a header cell (rough heuristic)
            let tag = "td"; // We'd need more context to determine th vs td
            let class_attr = config.td_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            format!("<{}{}>{}</{}>", tag, class_attr, content, tag)
        }
        
        mdast::Node::ThematicBreak(_) => {
            let class_attr = config.hr_class.as_ref()
                .map(|c| format!(" class=\"{}\"", c))
                .unwrap_or_default();
            
            format!("<hr{} />", class_attr)
        }
        
        mdast::Node::Break(_) => {
            "<br />".to_string()
        }
        
        mdast::Node::Delete(delete) => {
            let content = delete.children.iter()
                .map(|child| ast_to_html(child, config))
                .collect::<Vec<_>>()
                .join("");
            
            format!("<del>{}</del>", content)
        }
        
        mdast::Node::Html(html) => {
            // Pass through HTML as-is, we'll handle custom quotes in post-processing
            html.value.clone()
        }
        
        // Handle other node types by recursing into children or ignoring
        _ => {
            // For unhandled node types, try to extract children if possible
            String::new()
        }
    }
}

/// Escape HTML entities
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Post-process HTML to convert custom quote markup to styled spans
fn post_process_quotes(html: &str, config: &MarkdownConfig) -> String {
    let mut result = String::with_capacity(html.len());
    let mut chars = html.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '<' {
            // Check if this is the start of our custom quote tag
            let mut tag_start = String::new();
            tag_start.push(ch);
            
            // Read ahead to see if this is "<hearth-quote>"
            let mut temp_chars = chars.clone();
            let mut temp_tag = String::new();
            
            while let Some(&next_ch) = temp_chars.peek() {
                if next_ch == '>' {
                    temp_tag.push(temp_chars.next().unwrap());
                    break;
                } else {
                    temp_tag.push(temp_chars.next().unwrap());
                }
            }
            
            if temp_tag == "hearth-quote>" {
                // Found opening tag, consume it and look for the content and closing tag
                // Advance the main iterator past the opening tag
                for _ in 0..temp_tag.len() {
                    tag_start.push(chars.next().unwrap());
                }
                
                // Now collect everything until we find "</hearth-quote>"
                let mut quote_content = String::new();
                let mut found_closing = false;
                
                while let Some(content_ch) = chars.next() {
                    if content_ch == '<' {
                        // Check if this is the closing tag
                        let mut closing_tag = String::new();
                        closing_tag.push(content_ch);
                        
                        let mut temp_chars = chars.clone();
                        let mut temp_tag = String::new();
                        
                        while let Some(&next_ch) = temp_chars.peek() {
                            if next_ch == '>' {
                                temp_tag.push(temp_chars.next().unwrap());
                                break;
                            } else {
                                temp_tag.push(temp_chars.next().unwrap());
                            }
                        }
                        
                        if temp_tag == "/hearth-quote>" {
                            // Found closing tag, consume it
                            for _ in 0..temp_tag.len() {
                                closing_tag.push(chars.next().unwrap());
                            }
                            found_closing = true;
                            break;
                        } else {
                            // Not the closing tag, add it to content
                            quote_content.push(content_ch);
                        }
                    } else {
                        quote_content.push(content_ch);
                    }
                }
                
                if found_closing {
                    // Replace with styled span
                    let class_attr = config.quote_class.as_ref()
                        .map(|c| format!(" class=\"{}\"", c))
                        .unwrap_or_default();
                    
                    result.push_str(&format!("<span{}>{}</span>", class_attr, quote_content));
                } else {
                    // Malformed quote, just add the original tag
                    result.push_str(&tag_start);
                    result.push_str(&quote_content);
                }
            } else {
                // Not our custom tag, just add the '<' and continue
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let config = MarkdownConfig::default();
        let result = markdown_to_html("**bold** and *italic*", &config).unwrap();
        assert!(result.contains("<strong>") && result.contains("<em>"));
    }

    #[test]
    fn test_custom_quotes() {
        let config = MarkdownConfig {
            quote_class: Some("text-orange-500".to_string()),
            ..Default::default()
        };
        let result = markdown_to_html("This is \"quoted text\" here.", &config).unwrap();
        assert!(result.contains("class=\"text-orange-500\""));
        assert!(result.contains("&quot;quoted text&quot;"));
    }

    #[test]
    fn test_custom_quotes_with_default_class() {
        let config = MarkdownConfig::default();
        let result = markdown_to_html("Say \"hello world\" to everyone.", &config).unwrap();
        assert!(result.contains("class=\"text-orange-500\"")); // Default class
        assert!(result.contains("&quot;hello world&quot;"));
    }

    #[test]
    fn test_quotes_not_in_code_blocks() {
        let config = MarkdownConfig::default();
        let result = markdown_to_html("```\n\"this should not be quoted\"\n```", &config).unwrap();
        assert!(!result.contains("<span class=\"text-orange-500\">"));
        assert!(result.contains("&quot;this should not be quoted&quot;"));
    }

    #[test]
    fn test_quotes_not_in_inline_code() {
        let config = MarkdownConfig::default();
        let result = markdown_to_html("Here is `\"some code\"` with quotes.", &config).unwrap();
        assert!(!result.contains("<span class=\"text-orange-500\">"));
        assert!(result.contains("&quot;some code&quot;"));
    }

    #[test]
    fn test_mixed_quotes_and_markdown() {
        let config = MarkdownConfig::default();
        let result = markdown_to_html("**Bold** and \"quoted text\" with *italic*.", &config).unwrap();
        assert!(result.contains("<strong>Bold</strong>"));
        assert!(result.contains("<em>italic</em>"));
        assert!(result.contains("class=\"text-orange-500\""));
        assert!(result.contains("&quot;quoted text&quot;"));
    }

    #[test]
    fn test_quotes_with_escaped_content() {
        let config = MarkdownConfig::default();
        let result = markdown_to_html("Say \"hello \\\"world\\\"\" to everyone.", &config).unwrap();
        assert!(result.contains("class=\"text-orange-500\""));
        assert!(result.contains("&quot;hello"));
    }

    #[test]
    fn test_incomplete_quotes_ignored() {
        let config = MarkdownConfig::default();
        let result = markdown_to_html("This has an unclosed \" quote.", &config).unwrap();
        // Since the quote is incomplete, it should remain as a literal quote
        assert!(!result.contains("class=\"text-orange-500\""));
        assert!(result.contains("&quot;"));
    }

    #[test]
    fn test_empty_quotes_ignored() {
        let config = MarkdownConfig::default();
        let result = markdown_to_html("Empty quotes \"\" are ignored.", &config).unwrap();
        // Empty quotes should be ignored, remaining as literal quotes
        assert!(!result.contains("class=\"text-orange-500\""));
        assert!(result.contains("&quot;"));
    }

    #[test]
    fn test_quotes_across_lines_ignored() {
        let config = MarkdownConfig::default();
        let result = markdown_to_html("Quote spanning\n\"multiple\nlines\" should not work.", &config).unwrap();
        assert!(!result.contains("class=\"text-orange-500\""));
    }

    #[test]
    fn test_headings() {
        let config = MarkdownConfig {
            heading_class: Some("text-2xl".to_string()),
            ..Default::default()
        };
        let result = markdown_to_html("# Heading", &config).unwrap();
        assert!(result.contains("<h1 class=\"text-2xl\">"));
    }

    #[test]
    fn test_complex_code_and_quotes() {
        let config = MarkdownConfig::default();
        let content = r#"Here's some text with "quotes" and:

```rust
fn test() {
    println!("This should not be quoted");
}
```

And more "quotes" after the code block."#;
        
        let result = markdown_to_html(content, &config).unwrap();
        
        // Should have quotes before and after code block
        let quote_count = result.matches("class=\"text-orange-500\"").count();
        assert_eq!(quote_count, 2);
        
        // Should not quote inside code block (will be escaped but not styled)
        assert!(result.contains("&quot;This should not be quoted&quot;"));
    }

    #[test]
    fn test_try_parse_quote() {
        let mut chars = "hello world\"".chars().peekable();
        let result = try_parse_quote(&mut chars);
        assert_eq!(result, Some("\"hello world\"".to_string()));
    }

    #[test]
    fn test_try_parse_quote_with_escape() {
        let mut chars = r#"hello \"escaped\" world""#.chars().peekable();
        let result = try_parse_quote(&mut chars);
        assert_eq!(result, Some(r#""hello \"escaped\" world""#.to_string()));
    }

    #[test]
    fn test_try_parse_quote_multiline_fails() {
        let mut chars = "hello\nworld\"".chars().peekable();
        let result = try_parse_quote(&mut chars);
        assert_eq!(result, None);
    }

    #[test]
    fn test_try_parse_quote_empty() {
        let mut chars = "\"".chars().peekable();
        let result = try_parse_quote(&mut chars);
        assert_eq!(result, None);
    }
}