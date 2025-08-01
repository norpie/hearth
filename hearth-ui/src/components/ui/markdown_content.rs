//! Markdown content rendering component using hearth-core's AST-based markdown processor
//!
//! The MarkdownContent component provides rich text rendering for markdown content
//! using markdown-rs parser with custom AST to HTML conversion and styling options.

use dioxus::prelude::*;
use hearth_core::{markdown_to_html, MarkdownConfig};

/// Styling properties for different markdown elements
#[derive(Props, Clone, PartialEq)]
pub struct MarkdownContentProps {
    /// The markdown content to render
    pub content: String,

    /// Optional CSS classes to apply to the container
    #[props(default)]
    pub class: Option<String>,

    // Element-specific styling props
    /// CSS classes for heading elements (h1-h6)
    #[props(default)]
    pub heading_class: Option<String>,
    
    /// CSS classes for paragraph elements
    #[props(default)]
    pub paragraph_class: Option<String>,
    
    /// CSS classes for emphasis elements (em/italic)
    #[props(default)]
    pub italic_class: Option<String>,
    
    /// CSS classes for strong elements (strong/bold)
    #[props(default)]
    pub strong_class: Option<String>,
    
    /// CSS classes for link elements
    #[props(default)]
    pub link_class: Option<String>,
    
    /// CSS classes for blockquote elements
    #[props(default)]
    pub blockquote_class: Option<String>,
    
    /// CSS classes for code elements (inline)
    #[props(default)]
    pub code_class: Option<String>,
    
    /// CSS classes for preformatted code blocks
    #[props(default)]
    pub pre_class: Option<String>,
    
    /// CSS classes for unordered list elements
    #[props(default)]
    pub ul_class: Option<String>,
    
    /// CSS classes for ordered list elements
    #[props(default)]
    pub ol_class: Option<String>,
    
    /// CSS classes for list item elements
    #[props(default)]
    pub li_class: Option<String>,
    
    /// CSS classes for table elements
    #[props(default)]
    pub table_class: Option<String>,
    
    /// CSS classes for table header elements
    #[props(default)]
    pub th_class: Option<String>,
    
    /// CSS classes for table data elements
    #[props(default)]
    pub td_class: Option<String>,
    
    /// CSS classes for horizontal rule elements
    #[props(default)]
    pub hr_class: Option<String>,
    
    /// CSS classes for custom quote elements ("text")
    #[props(default)]
    pub quote_class: Option<String>,
}

/// MarkdownContent component for rendering markdown text with custom styling
#[component]
pub fn MarkdownContent(props: MarkdownContentProps) -> Element {
    // Create markdown configuration from props
    let config = MarkdownConfig {
        heading_class: props.heading_class.clone(),
        paragraph_class: props.paragraph_class.clone(),
        italic_class: props.italic_class.clone(),
        strong_class: props.strong_class.clone(),
        link_class: props.link_class.clone(),
        blockquote_class: props.blockquote_class.clone(),
        code_class: props.code_class.clone(),
        pre_class: props.pre_class.clone(),
        ul_class: props.ul_class.clone(),
        ol_class: props.ol_class.clone(),
        li_class: props.li_class.clone(),
        table_class: props.table_class.clone(),
        th_class: props.th_class.clone(),
        td_class: props.td_class.clone(),
        hr_class: props.hr_class.clone(),
        quote_class: props.quote_class.clone(),
    };
    
    // Convert markdown to HTML using core functionality
    let html_output = match markdown_to_html(&props.content, &config) {
        Ok(html) => html,
        Err(e) => {
            log::error!("Failed to parse markdown: {}", e);
            format!("<p>Error parsing markdown: {}</p>", e)
        }
    };
    
    let container_class = props.class.as_deref().unwrap_or("");
    
    rsx! {
        div {
            class: "{container_class}",
            dangerous_inner_html: "{html_output}",
        }
    }
}