use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Shadow, Theme, Vector};
use iced::gradient::Linear;
use iced::widget::button::Status;
use iced::widget::{button, container, row, scrollable, text, Button, Text};
use crate::enums::change_type::ChangeType;
use crate::enums::message::Message;
use crate::structs::file::File;

#[derive(Debug)]
pub struct FileTree {
    files: Vec<File>,
}

impl Default for FileTree {
    fn default() -> Self {
        FileTree::new()
    }
}

impl FileTree {

    pub fn new() -> Self {
        let sample = vec![
            File{change_type:ChangeType::Added,   title:"Added"    .into(), meta:"Manlajra  •  By Hassan, Mar 5".into(), icon:"●".into()},
            File{change_type:ChangeType::Deleted, title:"Deleted"  .into(), meta:"Manlajra  •  By Hassan, Mar 5".into(), icon:"●".into()},
            File{change_type:ChangeType::Deleted, title:"Deleted"  .into(), meta:"Manlajra  •  By Hassan, Mar 5".into(), icon:"●".into()},
            File{change_type:ChangeType::Deleted, title:"Deleted"  .into(), meta:"Manlajra  •  By Hassan, Mar 5".into(), icon:"●".into()},
            File{change_type:ChangeType::Deleted, title:"Deleted"  .into(), meta:"Manlajra  •  By Hassan, Mar 5".into(), icon:"●".into()},
            File{change_type:ChangeType::Modified,title:"Modified" .into(), meta:"Manlajra  •  By Hassan, Mar 5".into(), icon:"●".into()},
            File{change_type:ChangeType::Modified,title:"Modified" .into(), meta:"Hasssan  •  By Hassan, Mar 5".into(), icon:"△".into()},
            File{change_type:ChangeType::Added,   title:"Modified" .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"^".into()},
            File{change_type:ChangeType::Modified,title:"Modified" .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"●".into()},
            File{change_type:ChangeType::Deleted, title:"A foó"    .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"●".into()},
            File{change_type:ChangeType::Deleted, title:"Modified" .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"^".into()},
            File{change_type:ChangeType::Modified,title:"Modified" .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"○".into()},
            File{change_type:ChangeType::Deleted, title:"Modified" .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"□".into()},
        ];

        FileTree { files: sample }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::RowClicked(idx) => { println!("Row {idx} clicked"); },
            Message::AcceptAll                => { println!("Accept All"); }
            Message::ReviewChange(i)          => { println!("Review {i}"); }
            Message::IgnoreChange(i)          => { println!("Ignore {i}"); }
            Message::FinishReview             => { println!("Finish Review"); }
        }
    }

    fn get_text_element(&self, label: String, size: f32, color: Color) -> Text {
        let style = move |_theme: &Theme| { 
            text::Style { color: Some(color.clone()) }
        };
        text(label).size(size).style(style)
    }

    pub fn view(&self) -> Element<Message> {
        let text_primary = Color::from_rgb8(0xeb, 0xf4, 0xff);
        let text_secondary = Color::from_rgb8(0x6b, 0x72, 0x80);

        let accept_all_btn_style = move |theme: &Theme, status: Status| {
            let active_style = button::Style {
                background: Some(Background::Color(Color::from_rgb8(0x11, 0x14, 0x1b))),
                border: Border { color: Color::from_rgb8(0x07, 0x20, 0x27), width: 1.0, radius: 100.0.into() },
                shadow: Shadow { color: Color::from_rgba8(0, 0, 0, 115.0), offset: Vector::new(0.0, 2.0), blur_radius: 6.0 },
                text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            };

            let hovered_style = button::Style {
                background: Some(Background::Color(Color::from_rgb8(0x08, 0x34, 0x3e))),
                border: Border { color: Color::from_rgb8(0x07, 0x20, 0x27), width: 1.0, radius: 100.0.into() },
                shadow: Shadow { color: Color::from_rgba8(0, 0, 0, 153.0), offset: Vector::new(0.0, 3.0), blur_radius: 10.0 },
                text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            };
            match status {
                Status::Active => active_style,
                Status::Pressed => active_style,
                Status::Hovered => hovered_style,
                _ => button::primary(theme, status),
            }
        };

        let accept_all_btn = button(self.get_text_element("Accept All".to_string(), 14.0, Color::from_rgb8(0xdb, 0xea, 0xfe)))
            .padding([6,22])
            .style(accept_all_btn_style)
            .on_press(Message::AcceptAll);
        
        let header_container_content = row![
            self.get_text_element("All Changes".to_string(), 32.0, text_primary),
            row![].width(Length::Fill),
            accept_all_btn
        ]
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .spacing(20);

        let header = container(header_container_content)
            .padding(Padding::ZERO)
            .width(Length::Fill);

        // Analysis Summary with left border
        let analysis_container_content = iced::widget::column![
            self.get_text_element("ANALYSIS SUMMARY".to_string(), 14.4, Color::from_rgb8(0xFF, 0xCC, 0x4D)),
            self.get_text_element("Found 15 critical issues across multiple categories: 8 security vulnerabilities (SQL injection risks), 3 bugs (type mismatches and typos), 2 performance issues (N+1 queries and large file processing), 12 clean code violations (long methods, large classes, magic numbers), 8 architecture issues (business logic in resolvers, missing service layer), and 6 duplicate code patterns (repeated validation, CRUD operations, query building).".to_string(), 13.1, text_primary),
            self.get_text_element("Most critical are the SQL injection vulnerabilities from string concatenation in query building.".to_string(), 13.1, Color::from_rgb8(0xff,0x57,0x57)),
        ].spacing(4);

        let analysis_style = |_theme: &Theme| {
            container::Style {
                text_color: Default::default(),
                background: Some(Background::Color(Color::from_rgba8(255, 255, 255, 8.0))), // 0.03 * 255 ≈ 8
                border: Border {
                    color: Color::from_rgb8(0xff, 0x57, 0x57),
                    width: 0.0,
                    radius: 12.0.into()
                },
                shadow: Default::default(),
            }
        };

        let analysis = container(analysis_container_content)
            .padding(Padding::new(18.0).left(22.0)) // Extra left padding for border effect
            .style(analysis_style)
            .width(Length::Fill);

        // File rows
        let rows = self.files.iter().enumerate().fold(iced::widget::column![].spacing(2), move |col, (i,ch)| {
            let btn_text = Color::from_rgb8(0xdb, 0xea, 0xfe);
            let colour = match ch.change_type {
                ChangeType::Added    => Color::from_rgb8(0x7b,0xfd,0x6b),
                ChangeType::Modified => Color::from_rgb8(0xff,0x9f,0x4d),
                ChangeType::Deleted  => Color::from_rgb8(0xff,0x57,0x57),
            };

            let (label,msg) = match ch.change_type {
                ChangeType::Added => ("Review", Message::ReviewChange(i)),
                _                 => ("Ignore", Message::IgnoreChange(i)),
            };

            // Secondary button style
            let secondary_btn_style = move |theme: &Theme, status: Status| {
                let active_style = button::Style {
                    background: Some(Background::Color(Color::from_rgba8(255, 255, 255, 13.0))), // 0.05 * 255 ≈ 13
                    text_color: btn_text,
                    border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 100.0.into() },
                    ..Default::default()
                };

                let hovered_style = button::Style {
                    background: Some(Background::Color(Color::from_rgba8(255, 255, 255, 31.0))), // 0.12 * 255 ≈ 31
                    text_color: btn_text,
                    border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 100.0.into() },
                    ..Default::default()
                };

                match status {
                    Status::Active => active_style,
                    Status::Hovered => hovered_style,
                    Status::Pressed => active_style,
                    _ => button::primary(theme, status),
                }
            };

            let text_rows = row![
                self.get_text_element(ch.icon.to_string(), 11.2, colour).width(Length::Fixed(12.0)),
                self.get_text_element(ch.title.to_string(), 15.2, colour),
                self.get_text_element(ch.meta.to_string(), 13.1, text_secondary)
            ];

            let inner_rows = row![
                text_rows.spacing(14).align_y(Alignment::Center).width(Length::Fill),
                button(self.get_text_element(label.to_string(), 13.0, btn_text))
                    .padding([6,20])
                    .style(secondary_btn_style)
                    .on_press(msg)
            ];

            let inner = inner_rows.align_y(Alignment::Center).spacing(20);

            // Row hover style
            let row_btn_style = |theme: &Theme, status: Status| {
                let active_style = button::Style {
                    background: None,
                    border: Border { radius: 14.0.into(), ..Default::default() },
                    text_color: Color::TRANSPARENT,
                    ..Default::default()
                };

                let hovered_style = button::Style {
                    background: Some(Background::Color(Color::from_rgba8(255, 255, 255, 10.0))), // 0.04 * 255 ≈ 10
                    border: Border { radius: 14.0.into(), ..Default::default() },
                    text_color: Color::TRANSPARENT,
                    ..Default::default()
                };
                match status {
                    Status::Active => active_style,
                    Status::Pressed => active_style,
                    Status::Hovered => hovered_style,
                    _ => button::primary(theme, status),
                }
            };

            let row_btn: Button<Message> = button(inner)
                .padding([12,10])
                .style(row_btn_style)
                .on_press(Message::RowClicked(i));
            col.push(row_btn)
        });

        // Scrollable changes with max height
        let changes = scrollable(rows).height(Length::Fill);

        // Finish Review button
        let finish_review_btn_style = move |theme: &Theme, status: Status| {
            let active_style = button::Style {
                background: Some(Background::Color(Color::from_rgb8(0x0b, 0x0d, 0x12))),
                border: Border { color: Color::from_rgb8(0x07, 0x20, 0x27), width: 1.0, radius: 100.0.into() },
                shadow: Shadow { color: Color::from_rgba8(0, 0, 0, 115.0), offset: Vector::new(0.0, 2.0), blur_radius: 6.0 },
                text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            };

            let hovered_style = button::Style {
                background: Some(Background::Color(Color::from_rgb8(0x08, 0x34, 0x3e))),
                border: Border { color: Color::from_rgb8(0x07, 0x20, 0x27), width: 1.0, radius: 100.0.into() },
                shadow: Shadow { color: Color::from_rgba8(0, 0, 0, 153.0), offset: Vector::new(0.0, 3.0), blur_radius: 10.0 },
                text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            };
            match status {
                Status::Active => active_style,
                Status::Pressed => active_style,
                Status::Hovered => hovered_style,
                _ => button::primary(theme, status),
            }
        };

        let finish_review_btn = button(self.get_text_element("Finish Review".to_string(), 14.0, Color::from_rgb8(0xdb, 0xea, 0xfe)))
            .padding([6,22])
            .style(finish_review_btn_style)
            .on_press(Message::FinishReview);

        let footer = container(finish_review_btn)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right);

        // Main panel style
        let main_panel_style = |_theme: &Theme| {
            let linear = Linear {
                angle: iced::Radians(std::f32::consts::PI), // 180 degrees
                stops: [
                    Some(iced::gradient::ColorStop {
                        offset: 0.0,
                        color: Color::from_rgb8(0x11, 0x14, 0x1b) // #11141b
                    }),
                    Some(iced::gradient::ColorStop {
                        offset: 1.0,
                        color: Color::from_rgb8(0x0b, 0x0d, 0x12) // #0b0d12
                    }),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            };

            container::Style {
                background: Some(Background::from(linear)),
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 20.0.into() },
                shadow: Shadow { color: Color::from_rgba8(0, 0, 0, 153.0), offset: Vector::new(0.0, 6.0), blur_radius: 24.0 },
                ..Default::default()
            }
        };

        // Main panel with proper spacing matching HTML
        let panel = container(
            iced::widget::column![header, analysis, changes, footer]
                .spacing(28) // Matches HTML spacing
                .padding(Padding::new(48.0).left(56.0).right(56.0)) // Matches HTML padding
        )
            .style(main_panel_style)
            .width(Length::Shrink)
            .height(Length::Shrink);

        // Root background
        let root_bg_style = |_theme: &Theme| {
            let linear = Linear {
                angle: iced::Radians(0.0),
                stops: [
                    Some(iced::gradient::ColorStop { offset: 0.0, color: Color::from_rgb8(0x24, 0x24, 0x3e) }),
                    Some(iced::gradient::ColorStop { offset: 0.5, color: Color::from_rgb8(0x30, 0x2b, 0x63) }),
                    Some(iced::gradient::ColorStop { offset: 1.0, color: Color::from_rgb8(0x0f, 0x0c, 0x29) }),
                    None,
                    None,
                    None,
                    None,
                    None,
                ]
            };

            container::Style {
                background: Some(Background::from(linear)),
                ..Default::default()
            }
        };

        // Root container matching HTML centering (80% width/height)
        container(
            container(panel)
                .width(Length::FillPortion(8)) // 80% width
                .height(Length::FillPortion(8)) // 80% height
                .center_x(Length::Fill)
                .center_y(Length::Fill)
        )
            .style(root_bg_style)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}