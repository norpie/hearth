use crate::views::design::showcase::{ComponentShowcase, ShowcaseVariant};
use dioxus::prelude::*;

#[component]
pub fn TypographyShowcase() -> Element {
    rsx! {
            ComponentShowcase {
                name: "Typography".to_string(),
                description: "Typography utility classes for consistent text styling".to_string(),
                basic_usage: r#"rsx! {
    div {
        h1 { class: "prose-h1", "Main Title" }
        p { class: "prose-p", "Paragraph text" }
        h2 { class: "prose-h2", "Subtitle" }
    }
}"#.to_string(),
                with_props_usage: "rsx! {\n    div {\n        p { class: \"prose-lead\", \"Lead paragraph text\" }\n        blockquote { class: \"prose-blockquote\", \"Quote text\" }\n        ul { class: \"prose-ul\",\n            li { \"List item\" }\n        }\n        a { class: \"prose-link\", href: \"#\", \"Link text\" }\n    }\n}".to_string(),
                ShowcaseVariant {
                    title: "Full Showcase".to_string(),
                div {
     h1 {
      class: "prose-h1",
      "Taxing Laughter: The Joke Tax Chronicles"
     }
     p {
      class: "prose-lead",
      r#"Once upon a time, in a far-off land, there was a very lazy king who spent all day lounging on his throne. One day, his advisors came to him with a problem: the kingdom was running out of money."#
     }
    h2 {
      class: "prose-h2",
      "The King's Plan"
     }
     p {
      class: "prose-p",
      r#"The king thought long and hard, and finally came up with "#
      a { class: "prose-link", href: "##", "a brilliant plan" }
      r#" : he would tax the jokes in the kingdom."#
     }
     blockquote {
      class: "prose-blockquote",
      r#""After all," he said, "everyone enjoys a good joke, so it's only fair that they should pay for the privilege.""#
     }
     h3 {
      class: "prose-h3",
      "The Joke Tax"
     }
     p {
      class: "prose-p",
      r#"The king's subjects were not amused. They grumbled and complained, but the king was firm:"#
     }
     ul {
      class: "prose-ul",
      li { "1st level of puns: 5 gold coins" }
      li { "2nd level of jokes: 10 gold coins" }
      li { "3rd level of one-liners : 20 gold coins" }
     }
     p {
      class: "prose-p",
      r#"As a result, people stopped telling jokes, and the kingdom fell into a gloom. But there was one person who refused to let the king's foolishness get him down: a court jester named Jokester."#
     }
     h3 {
      class: "prose-h3",
      "Jokester's Revolt"
     }
     p {
      class: "prose-p",
      r#"Jokester began sneaking into the castle in the middle of the night and leaving jokes all over the place: under the king's pillow, in his soup, even in the royal toilet. The king was furious, but he couldn't seem to stop Jokester."#
     }
     p {
      class: "prose-p",
      r#"And then, one day, the people of the kingdom discovered that the jokes left by Jokester were so funny that they couldn't help but laugh. And once they started laughing, they couldn't stop."#
     }
     h3 {
      class: "prose-h3",
      "The People's Rebellion"
     }
     p {
      class: "prose-p",
      r#"The people of the kingdom, feeling uplifted by the laughter, started to tell jokes and puns again, and soon the entire kingdom was in on the joke."#
     }
     div {
      class: "prose-table-wrapper",
      table {
       class: "prose-table",
       thead {
        tr {
         class: "prose-thead-tr",
         th {
          class: "prose-th",
      "King's Treasury"
         }
         th {
          class: "prose-th",
      "People's happiness"
         }
        }
       }
       tbody {
        tr {
         class: "prose-tbody-tr",
         td {
          class: "prose-td",
      "Empty"
         }
         td {
          class: "prose-td",
      "Overflowing"
         }
        }
        tr {
         class: "prose-tbody-tr prose-tbody-tr-even",
         td {
          class: "prose-td",
      "Modest"
         }
         td {
          class: "prose-td",
      "Satisfied"
         }
        }
        tr {
         class: "prose-tbody-tr",
         td {
          class: "prose-td",
      "Full"
         }
         td {
          class: "prose-td",
      "Ecstatic"
         }
        }
       }
      }
     }
     p {
      class: "prose-p",
      r#"The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax. Jokester was declared a hero, and the kingdom lived happily ever after."#
     }
     p {
      class: "prose-p",
      r#"The moral of the story is: never underestimate the power of a good laugh and always be careful of bad ideas."#
     }
    }
                }
            }
        }
}
