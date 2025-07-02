use iced::widget::{button, column, container, row, scrollable, text, Button};
use iced::{alignment::Alignment, executor, theme, Application, Background, Border, Color, Command, Element, Length, Padding, Settings, Shadow, Size, Theme, Vector};
use iced::gradient::Linear;
use iced::window::{self, Id};

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window = window::Settings {
        size: Size::new(1600.0, 1200.0),
        decorations: true,  // show native title bar
        transparent: false, // disable transparency
        resizable: true,
        ..Default::default()
    };

    ChangesPanel::run(settings)
}

#[derive(Debug, Clone, Copy)]
enum ChangeType { Added, Modified, Deleted }

#[derive(Debug, Clone)]
struct ChangeItem { change_type: ChangeType, title: String, meta: String, icon: String }

#[derive(Default)]
struct ChangesPanel { changes: Vec<ChangeItem> }

#[derive(Debug, Clone)]
enum Message {
    AcceptAll,
    ReviewChange(usize),
    IgnoreChange(usize),
    FinishReview,
    Close,
    Minimize,
    ToggleMax,
    RowClicked(usize),
}

impl Application for ChangesPanel {
    type Executor = executor::Default;
    type Message  = Message;
    type Theme    = Theme;
    type Flags    = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        let sample = vec![
            ChangeItem{change_type:ChangeType::Added,   title:"Added"    .into(), meta:"Manlajra  •  By Hassan, Mar 5".into(), icon:"●".into()},
            ChangeItem{change_type:ChangeType::Deleted, title:"Deleted"  .into(), meta:"Manlajra  •  By Hassan, Mar 5".into(), icon:"●".into()},
            ChangeItem{change_type:ChangeType::Modified,title:"Modified" .into(), meta:"Manlajra  •  By Hassan, Mar 5".into(), icon:"●".into()},
            ChangeItem{change_type:ChangeType::Modified,title:"Modified" .into(), meta:"Hasssan  •  By Hassan, Mar 5".into(), icon:"△".into()},
            ChangeItem{change_type:ChangeType::Added,   title:"Modified" .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"^".into()},
            ChangeItem{change_type:ChangeType::Modified,title:"Modified" .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"●".into()},
            ChangeItem{change_type:ChangeType::Deleted, title:"A foó"    .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"●".into()},
            ChangeItem{change_type:ChangeType::Deleted, title:"Modified" .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"^".into()},
            ChangeItem{change_type:ChangeType::Modified,title:"Modified" .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"○".into()},
            ChangeItem{change_type:ChangeType::Deleted, title:"Modified" .into(), meta:"Mar 5  •  By Hassan, Mar 5".into(), icon:"□".into()},
        ];
        (Self { changes: sample }, Command::none())
    }

    fn title(&self) -> String { "All Changes Panel".into() }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use Message::*;
        match message {
            RowClicked(idx) => { println!("Row {idx} clicked"); Command::none() },
            Close       => window::close::<Self::Message>(window::Id::MAIN),
            Minimize    => window::minimize::<Self::Message>(window::Id::MAIN, true),
            ToggleMax   => window::toggle_maximize::<Self::Message>(window::Id::MAIN),

            // demo handlers
            AcceptAll                => { println!("Accept All"); Command::none() }
            ReviewChange(i)          => { println!("Review {i}"); Command::none() }
            IgnoreChange(i)          => { println!("Ignore {i}"); Command::none() }
            FinishReview             => { println!("Finish Review"); Command::none() }
        }
    }

    fn view(&self) -> Element<Message> {
        let header = container(
            row![
                text("All Changes").size(24).style(Color::from_rgb8(0xdb,0xea,0xfe)),
                row![].width(Length::Fill),
                button(text("Accept All").size(14).style(Color::from_rgb8(0xdb,0xea,0xfe)))
                    .padding([6,22])
                    .style(theme::Button::Custom(Box::new(AcceptAllButton)))
                    .on_press(Message::AcceptAll)
            ]
                .align_items(Alignment::Center)
                .width(Length::Fill)
                .spacing(20)
        )
            .padding([0,0,0,0])
            .style(theme::Container::Custom(Box::new(HeaderBar)))
            .width(Length::Fill);

        let analysis = container(
            column![
                text("ANALYSIS SUMMARY").size(14).style(Color::from_rgb8(0xff,0xcc,0x4d)),
                text("Found 15 critical issues across multiple categories: 8 security vulnerabilities (SQL injection risks), 3 bugs (type mismatches and typos), 2 performance issues (N+1 queries and large file processing), 12 clean code violations (long methods, large classes, magic numbers), 8 architecture issues (business logic in resolvers, missing service layer), and 6 duplicate code patterns (repeated validation, CRUD operations, query building).")
                    .size(13)
                    .style(Color::from_rgb8(0xeb,0xf4,0xff)),
                text("Most critical are the SQL injection vulnerabilities from string concatenation in query building.")
                    .size(13)
                    .style(Color::from_rgb8(0xff,0x57,0x57)),
            ]
                .spacing(4)
        )
            .padding(18)
            .style(theme::Container::Custom(Box::new(AnalysisSummary)))
            .width(Length::Fill);

        let rows = self.changes.iter().enumerate().fold(column![].spacing(2), |col,(i,ch)| {
            let colour = match ch.change_type {
                ChangeType::Added    => Color::from_rgb8(0x7b,0xfd,0x6b),
                ChangeType::Modified => Color::from_rgb8(0xff,0x9f,0x4d),
                ChangeType::Deleted  => Color::from_rgb8(0xff,0x57,0x57),
            };
            let (label,msg) = match ch.change_type {
                ChangeType::Added => ("Review", Message::ReviewChange(i)),
                _                 => ("Ignore", Message::IgnoreChange(i)),
            };

            let inner = row![
                row![text(&ch.icon).style(colour).size(11).width(Length::Fixed(20.0)),
                     text(&ch.title).style(colour).size(15),
                     text(&ch.meta).style(Color::from_rgb8(0x6b,0x72,0x80)).size(13)]
                    .spacing(14)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
                button(text(label).size(13).style(Color::from_rgb8(0xdb,0xea,0xfe)))
                    .padding([6,20])
                    .style(theme::Button::Custom(Box::new(SecondaryButton)))
                    .on_press(msg)
            ]
                .align_items(Alignment::Center)
                .spacing(20);

            let row_btn: Button<Message> = button(inner)
                .padding([12,10])
                .style(theme::Button::Custom(Box::new(ChangeRowButton)))
                .on_press(Message::RowClicked(i));

            col.push(row_btn)
        });

        let changes = scrollable(rows).height(Length::Shrink);

        let footer = container(
            button(text("Finish Review").size(14).style(Color::from_rgb8(0xdb,0xea,0xfe)))
                .padding([6,22])
                .style(theme::Button::Custom(Box::new(FinishReviewButton)))
                .on_press(Message::FinishReview)
        )
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right);

        let panel = container(
            column![header, analysis, changes, footer]
                .spacing(24)
                .padding(Padding::from([32,28,24,28]))
        )
            .style(theme::Container::Custom(Box::new(MainPanel)))
            .width(Length::Shrink)
            .height(Length::Shrink);

        container(
            container(panel)
                .style(theme::Container::Custom(Box::new(MainPanel)))
                .width(Length::Fill)
                .height(Length::Fill)
        )
            .style(theme::Container::Custom(Box::new(RootBg)))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from([50,150,50,150]))
            .into()
    }
}

struct RootBg;
struct MainPanel;
struct HeaderBar;
struct AnalysisSummary;
struct AcceptAllButton;
struct SecondaryButton;
struct FinishReviewButton;
struct ChangeRowButton;

impl container::StyleSheet for RootBg {
    type Style = Theme;
    fn appearance(&self, _: &Self::Style) -> container::Appearance {

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

        container::Appearance {
            background: Some(Background::from(linear)),
            ..Default::default()
        }
    }
}

impl container::StyleSheet for MainPanel {
    type Style = Theme;
    fn appearance(&self, _: &Self::Style) -> container::Appearance {
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

        container::Appearance {
            background: Some(Background::from(linear)),
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 20.0.into() },
            shadow: Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.6), offset: Vector::new(0.0, 6.0), blur_radius: 24.0 },
            ..Default::default()
        }
    }
}

impl container::StyleSheet for HeaderBar {
    type Style = Theme;
    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x11, 0x14, 0x1b))),
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 20.0.into() },
            ..Default::default()
        }
    }
}

impl container::StyleSheet for AnalysisSummary {
    type Style = Theme;
    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.03))),
            border: Border { color: Color::from_rgb8(0xff, 0x57, 0x57), width: 4.0, radius: 12.0.into() },
            ..Default::default()
        }
    }
}

impl button::StyleSheet for ChangeRowButton {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,                       // transparent
            border: Border { radius: 14.0.into(), ..Default::default() },
            text_color: Color::TRANSPARENT,         // inner labels keep their own colour
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let mut a = self.active(style);
        a.background = Some(Background::Color(Color::from_rgba(255.0,255.0,255.0,0.04))); // subtle highlight
        a
    }
}

impl button::StyleSheet for AcceptAllButton {
    type Style = Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x11, 0x14, 0x1b))),
            border: Border { color: Color::from_rgb8(0x07, 0x20, 0x27), width: 1.0, radius: 100.0.into() },
            shadow: Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.45), offset: Vector::new(0.0, 2.0), blur_radius: 6.0 },
            text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            ..Default::default()
        }
    }
    fn hovered(&self, s: &Self::Style) -> button::Appearance {
        let mut a = self.active(s);
        a.background = Some(Background::Color(Color::from_rgb8(0x08, 0x34, 0x3e)));
        a.shadow = Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.6), offset: Vector::new(0.0, 3.0), blur_radius: 10.0 };
        a
    }
}

impl button::StyleSheet for SecondaryButton {
    type Style = Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.05))),
            text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 100.0.into() },
            ..Default::default()
        }
    }
    fn hovered(&self, s: &Self::Style) -> button::Appearance {
        let mut a = self.active(s);
        a.background = Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.12)));
        a
    }
}

impl button::StyleSheet for FinishReviewButton {
    type Style = Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x0b, 0x0d, 0x12))),
            border: Border { color: Color::from_rgb8(0x07, 0x20, 0x27), width: 1.0, radius: 100.0.into() },
            shadow: Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.45), offset: Vector::new(0.0, 2.0), blur_radius: 6.0 },
            text_color: Color::from_rgb8(0xdb, 0xea, 0xfe),
            ..Default::default()
        }
    }
    fn hovered(&self, s: &Self::Style) -> button::Appearance {
        let mut a = self.active(s);
        a.background = Some(Background::Color(Color::from_rgb8(0x08, 0x34, 0x3e)));
        a.shadow = Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.6), offset: Vector::new(0.0, 3.0), blur_radius: 10.0 };
        a
    }
}
