use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Shadow, Theme, Vector};
use iced::gradient::Linear;
use iced::widget::button::Status;
use iced::widget::{button, container, row, scrollable, text, Button};
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

    pub fn view(&self) -> Element<Message> {
        let main_white_color = Color::from_rgb8(0xdb, 0xea, 0xfe);
        let text_color = |color: Color | {
            move |_theme: &Theme| { text::Style { color: Some(color.clone()) } }
        };

        let get_text_element = |label: String, size: f32, color: Color | {
            text(label).size(size).style(text_color(color))
        };

        let accept_all_btn_style = |theme: &Theme, status: Status| {
            let active_style = button::Style {
                background: Some(Background::Color(Color::from_rgb8(0x11, 0x14, 0x1b))),
                border: Border { color: Color::from_rgb8(0x07, 0x20, 0x27), width: 1.0, radius: 100.0.into() },
                shadow: Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.45), offset: Vector::new(0.0, 2.0), blur_radius: 6.0 },
                text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            };

            let hovered_style = button::Style {
                background: Some(Background::Color(Color::from_rgb8(0x08, 0x34, 0x3e))),
                border: Border { color: Color::from_rgb8(0x07, 0x20, 0x27), width: 1.0, radius: 100.0.into() },
                shadow: Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.6), offset: Vector::new(0.0, 3.0), blur_radius: 10.0 },
                text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            };
            match status {
                Status::Active => active_style,
                Status::Pressed => active_style,
                Status::Hovered => hovered_style,
                _ => button::primary(theme, status),
            }
        };

        let accept_all_btn = button(get_text_element("Accept All".to_string(), 14.0, main_white_color))
            .padding([6,22])
            .style(accept_all_btn_style)
            .on_press(Message::AcceptAll);

        let header_container_style = |_theme: &Theme| {
            container::Style {
                text_color: None,
                background: Some(Background::Color(Color::from_rgb8(0x11, 0x14, 0x1b))),
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 20.0.into() },
                shadow: Default::default(),
            }
        };

        let header_container_content = row![get_text_element("All Changes".to_string(), 24.0, main_white_color), row![].width(Length::Fill), accept_all_btn]
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .spacing(20);

        let header = container(header_container_content).padding(Padding::ZERO).style(header_container_style).width(Length::Fill);

        let analysis_container_content = iced::widget::column![
        get_text_element("ANALYSIS SUMMARY".to_string(), 14.0, Color::from_rgb8(0xff,0xcc,0x4d)),
        get_text_element("Found 15 critical issues across multiple categories: 8 security vulnerabilities (SQL injection risks), 3 bugs (type mismatches and typos), 2 performance issues (N+1 queries and large file processing), 12 clean code violations (long methods, large classes, magic numbers), 8 architecture issues (business logic in resolvers, missing service layer), and 6 duplicate code patterns (repeated validation, CRUD operations, query building).".to_string(), 13.0, Color::from_rgb8(0xeb,0xf4,0xff)),
        get_text_element("Most critical are the SQL injection vulnerabilities from string concatenation in query building.".to_string(), 13.0, Color::from_rgb8(0xff,0x57,0x57)),
    ].spacing(4);

        let analysis_style = |_theme: &Theme| {
            container::Style {
                text_color: None,
                background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.03))),
                border: Border { color: Color::from_rgb8(0xff, 0x57, 0x57), width: 4.0, radius: 12.0.into() },
                shadow: Default::default(),
            }
        };

        let analysis = container(analysis_container_content).padding(18).style(analysis_style).width(Length::Fill);

        let rows = self.files.iter().enumerate().fold(iced::widget::column![].spacing(2), |col, (i,ch)| {

            let colour = match ch.change_type {
                ChangeType::Added    => Color::from_rgb8(0x7b,0xfd,0x6b),
                ChangeType::Modified => Color::from_rgb8(0xff,0x9f,0x4d),
                ChangeType::Deleted  => Color::from_rgb8(0xff,0x57,0x57),
            };

            let (label,msg) = match ch.change_type {
                ChangeType::Added => ("Review", Message::ReviewChange(i)),
                _                 => ("Ignore", Message::IgnoreChange(i)),
            };

            let secondary_btn_style = |theme: &Theme, status: Status| {
                let active_style = button::Style {
                    background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.05))),
                    text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
                    border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 100.0.into() },
                    ..Default::default()
                };

                let hovered_style = button::Style {
                    background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.12))),
                    text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
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
            get_text_element(ch.icon.to_string(), 11.0, colour).width(Length::Fixed(20.0)),
            get_text_element(ch.title.to_string(), 15.0, colour),
            get_text_element(ch.meta.to_string(), 13.0, Color::from_rgb8(0x6b,0x72,0x80))
        ];

            let inner_rows = row![
            text_rows.spacing(14).align_y(Alignment::Center).width(Length::Fill),
            button(get_text_element(label.to_string(), 13.0, Color::from_rgb8(0xdb,0xea,0xfe))).padding([6,20]).style(secondary_btn_style).on_press(msg)
        ];

            let inner = inner_rows.align_y(Alignment::Center).spacing(20);

            let row_btn_style = |theme: &Theme, status: Status| {
                let active_style = button::Style {
                    background: None,
                    border: Border { radius: 14.0.into(), ..Default::default() },
                    text_color: Color::TRANSPARENT,
                    ..Default::default()
                };

                let hovered_style = button::Style {
                    background: Some(Background::Color(Color::from_rgba(255.0,255.0,255.0,0.04))),
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

            let row_btn: Button<Message> = button(inner).padding([12,10]).style(row_btn_style).on_press(Message::RowClicked(i));
            col.push(row_btn)
        });

        let changes = scrollable(rows).height(Length::Shrink);

        let finish_review_btn_style = |theme: &Theme, status: Status| {
            let active_style = button::Style {
                background: Some(Background::Color(Color::from_rgb8(0x0b, 0x0d, 0x12))),
                border: Border { color: Color::from_rgb8(0x07, 0x20, 0x27), width: 1.0, radius: 100.0.into() },
                shadow: Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.45), offset: Vector::new(0.0, 2.0), blur_radius: 6.0 },
                text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            };

            let hovered_style = button::Style {
                background: Some(Background::Color(Color::from_rgb8(0x08, 0x34, 0x3e))),
                border: Border { color: Color::from_rgb8(0x07, 0x20, 0x27), width: 1.0, radius: 100.0.into() },
                shadow: Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.6), offset: Vector::new(0.0, 3.0), blur_radius: 10.0 },
                text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            };
            match status {
                Status::Active => active_style,
                Status::Pressed => active_style,
                Status::Hovered => hovered_style,
                _ => button::primary(theme, status),
            }

        };
        let finish_review_btn = button(get_text_element("Finish Review".to_string(), 14.0, Color::from_rgb8(0xdb,0xea,0xfe))).padding([6,22]).style(finish_review_btn_style).on_press(Message::FinishReview);
        let footer = container(finish_review_btn).width(Length::Fill).align_x(iced::alignment::Horizontal::Right);

        let main_panel_style = |_theme: &Theme| {
            let linear = Linear {
                angle: iced::Radians(std::f32::consts::PI), // 180 degrees = π radians
                stops: [
                    Some(iced::gradient::ColorStop {
                        offset: 0.0,
                        color: Color::from_rgb8(0x11, 0x14, 0x1b) // #11141b (panel-bg-start)
                    }),
                    Some(iced::gradient::ColorStop {
                        offset: 1.0,
                        color: Color::from_rgb8(0x0b, 0x0d, 0x12) // #0b0d12 (panel-bg-end)
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
                shadow: Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.6), offset: Vector::new(0.0, 6.0), blur_radius: 24.0 },
                ..Default::default()
            }
        };

        let panel = container(iced::widget::column![header, analysis, changes, footer].spacing(24).padding(Padding::new(0.0).top(32).bottom(28).left(24).right(28)))
            .style(main_panel_style)
            .width(Length::Shrink)
            .height(Length::Shrink);

        let root_bg_style = |_theme: &Theme| {
            let linear = Linear {
                angle: iced::Radians(0.0),
                stops: [
                    Some(iced::gradient::ColorStop { offset: 0.0, color: Color::from_rgb8(0x12, 0x16, 0x1f) }),
                    Some(iced::gradient::ColorStop { offset: 0.7, color: Color::from_rgb8(0x09, 0x0b, 0x10) }),
                    None,
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

        container(container(panel).style(main_panel_style).width(Length::Fill).height(Length::Fill))
            .style(root_bg_style)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::new(0.0).top(50).bottom(50).left(150).right(150))
            .into()
    }
}

