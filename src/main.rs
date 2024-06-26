#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{fa_brands_icons::FaGithub, fi_icons::FiMail},
    Icon, IconShape,
};
use tracing::Level;

/*
#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/education")]
    Education {},
    #[route("/certifications")]
    Certifications {},
    #[route("/experience")]
    Experience {},
    #[route("/projects")]
    Projects {},
    #[route("/contributions")]
    Contributions {},
    #[route("/")]
    #[redirect("/:_path", |_path: String| Route::Home {})]
    Home {}
}

*/

#[derive(Clone, Copy, PartialEq, Debug)]
enum Page {
    About,
    Education,
    Certifications,
    Experience,
    Projects,
    Contributions,
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    let page = use_signal(|| Page::About);
    let mut animation = use_signal(|| false);
    let (content, background_image) = match page() {
        Page::About => about(),
        Page::Education => education(),
        Page::Certifications => certifications(),
        Page::Experience => experience(),
        Page::Projects => projects(),
        Page::Contributions => contributions(),
    };
    let animation_name = if animation() { "load" } else { "load2" };
    let view = rsx! {
        div {
            style: "
                min-height: 100%;
                display: flex;
                justify-content: center;
                margin-left: var(--nav-width);
                animation-name: {animation_name};
                animation-duration: 1s;
                background-image: url('{background_image}');
                background-size: cover;
            ",
            div {
                style: "
                min-height: 100%;
                max-width: 1000px;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                margin: 10px;
                padding-top: auto;
                padding-bottom: auto;
            ",
                div {
                    {content}
                }
            }
        }
    };
    rsx! {
        Nav { page, animation }
        {view}
    }
}

#[component]
fn Nav(page: Signal<Page>, animation: Signal<bool>) -> Element {
    let items = [
        ("About", Page::About),
        ("Education", Page::Education),
        ("Certifications", Page::Certifications),
        ("Experience", Page::Experience),
        ("Projects", Page::Projects),
        ("Contributions", Page::Contributions),
    ];
    rsx! {
        div {
            style: "
                position: absolute;
                top: 0;
                left: 0;
                width: var(--nav-width);
                display: flex;
                flex-direction: column;
            ",
            NavTitle { page, animation }
            for (label, to) in items {
                NavItem { page, animation, label, to, }
            }
            Contact {}
        }
    }
}

#[component]
fn NavTitle(page: Signal<Page>, animation: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "hover",
            style: "
                width: 100%;
                font-weight: bold;
                border-style: none none solid none;
                border-width: thin;
                border-color: var(--accent-color);
                display: flex;
                align-items: center;
            ",
            onclick: move |_| {
                page.with_mut(|x| {
                    if *x != Page::About {
                        *x = Page::About;
                        animation.with_mut(|x| { *x = !*x; });
                    }
                });
            },
            span {
                style: "margin: 10px; text-align: center;",
                "Charles Earp",
            }
        }
    }
}

#[component]
fn NavItem(page: Signal<Page>, animation: Signal<bool>, label: &'static str, to: Page) -> Element {
    rsx! {
        button {
            class: "hover",
            style: "
                width: 100%;
                display: flex;
                align-items: center;
                border-style: none none solid none;
                border-color: var(--accent-color);
                border-width: thin;
            ",
            onclick: move |_| {
                page.with_mut(|x| {
                    if *x != to {
                        *x = to;
                        animation.with_mut(|x| { *x = !*x; });
                    }
                });
            },
            span { style: "margin: 10px;", {label} }
        }
    }
}

#[component]
fn Contact() -> Element {
    rsx! {
        div {
            style: "width: 100%; display: flex; justify-content: space-around; align-items: center;",
            ContactIcon {
                icon: FiMail,
                href: "mailto:charles.r.earp@gmail.com",
            }
            ContactIcon {
                icon: FaGithub,
                href: "https://www.github.com/charles-r-earp",
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ContactIconProps<T: IconShape + Clone + PartialEq + 'static> {
    icon: T,
    href: &'static str,
}

fn ContactIcon<T: IconShape + Clone + PartialEq + 'static>(props: ContactIconProps<T>) -> Element {
    let ContactIconProps { icon, href } = props;
    let size = 40;
    rsx! {
        a {
            class: "hover",
            style: "margin: 10px;",
            href,
            Icon {
                width: size,
                height: size,
                icon,
            }
        }
    }
}

fn about() -> (Element, &'static str) {
    // https://www.pixelstalk.net/wp-content/uploads/2016/11/Earth-From-Space-Wallpapers-HD-Desktop.jpg"
    let background_image = "Earth-From-Space-HD-Backgrounds.jpg";
    let content = rsx! {};
    (content, background_image)
}

fn education() -> (Element, &'static str) {
    let background_image = "Memorial_Union_at_Oregon_State_University.jpg";
    let content = rsx! {
        Card {
            href: "https://oregonstate.edu",
            span {
                style: "font-size: 2em; font-weight: bold;",
                "BS Mechanical Engineering",
            }
            span {
                style: "font-style: italic;",
                "Oregon State University"
            }
        }
        br {}
        Card {
            href: "https://www.global-formula-racing.com",
            span {
                style: "font-size: 2em; font-weight: bold;",
                "Capstone Senior Design",
            }
            span {
                style: "font-style: italic;",
                "Global Formula Racing"
            }
            p {
                "As part of Capstone Senior Design, worked on the Data Acquisition System of a Racecar. This
                began with the evaluation of previous designs, and determining what data was to be measured,
                presenting the proposal, all the way through fabrication and assembly. For example, a new GPS
                system was selected and was sponsored, which has inch level precision. Also, a suite of
                proximity sensors (for the chassis and aerodynamics) were acquired and custom mounts were
                designed and fabricated, including custom wiring and potting."
            }
        }
    };
    (content, background_image)
}

fn certifications() -> (Element, &'static str) {
    // https://wallpaperaccess.com/engineering-drawing drafting
    let background_image = "drafting_cropped.jpg";
    let content = rsx! {
        Card {
            href: "https://www.oregon.gov/OSBEELS",
            title: "Engineering Intern",
            subtitle: "Oregon State Board of Examiners for Engineering & Land Surveying",
        }
    };
    (content, background_image)
}

fn experience() -> (Element, &'static str) {
    // https://wallpaperaccess.com/semiconductor colorful
    let background_image = "semiconductors.jpg";
    let content = rsx! {
        Card {
            title: "Manufacturing Technician",
            subtitle: "Retronix Semiconductor",
            p {
                "Manages and monitors semiconductor manufacturing equipment and automation system for
                Intel from a remote command center. Ensures that production is running smoothly, operates
                equipment remotely and escalates any delays or equipment issues as appropriate. Documents
                potential automation issues to improve utilization. Maintains quality control in accordance with
                engineering instructions and propagates WIP through manufacturing stages, coordinating with
                other manufacturing groups as appropriate."
            }
        }
        br {}
        Card {
            title: "Field Service Engineer",
            subtitle: "Tokyo Electron",
            p {
                "Provides Intel with maintenance support for COT / DEV tools, such as ACT12, Lithius,
                and Lithius Pro V. Routinely leads 2 person work groups in coordination with our customer Intel,
                to complete both routine maintenance, upgrades, and troubleshooting of tool down situations,
                escalating to knowledgeable team members when necessary. Additionally performs
                modifications based on schematics to replace hardware, electronics, or chemical / gas lines and
                components, and or update firmware and or software."
            }
        }
    };
    (content, background_image)
}

fn projects() -> (Element, &'static str) {
    // https://www.flickr.com/photos/57892278@N02/6976843469/
    let background_image = "Matrix.jpg";
    let content = rsx! {
        Card {
            href: "https://www.github.com/charles-r-earp/charles-r-earp.github.io",
            title: "Portfolio Website",
            p {
                "This website, built with Dioxus."
            }
        }
        br {}
        Card {
            href: "https://www.github.com/charles-r-earp/autograph",
            title: "autograph",
            subtitle: "A machine learning library for Rust.",
            p {
                "Built a functional neural network library from scratch, with support for both CPU and GPU
                execution. Most of the code is Rust, with some C++, as well as CUDA and OpenCL.
                Experimented with OpenCL and ROCm backends in order to target AMD as well. Features
                include Dense, Convolutional, Pooling, and ReLU layers, Stochastic Gradient Descent
                optimizer, and the ability to checkpoint and save the model parameters."
            }
        }
        br {}
        Card {
            href: "https://www.github.com/charles-r-earp/krnl",
            title: "krnl",
            subtitle: "Safe, portable, high performance compute (GPGPU) kernels.",
            p {
                "Similar functionality to CUDA and OpenCL. Kernels are written inline, entirely in Rust, using
                module and kernel proc macros. Simple iterator patterns can be implemented without unsafe.
                krnlc locates modules and compiles with rust-gpu’s spirv-builder."
            }
        }
    };
    (content, background_image)
}

fn contributions() -> (Element, &'static str) {
    // http://upload.wikimedia.org/wikipedia/commons/a/a2/Polycyclic_Aromatic_Hydrocarbons_In_Space.jpg
    let background_image = "Polycyclic_Aromatic_Hydrocarbons_In_Space.jpg";
    let content = rsx! {
        Card {
            href: "https://github.com/rust-lang/cargo/pulls?q=is%3Apr+author%3Acharles-r-earp",
            title: "Cargo",
            subtitle: "The Rust package manager.",
            p {
                "Added workspace members to cargo metadata command."
            }
        }
        br {}
        Card {
            href: "https://github.com/EmbarkStudios/rust-gpu/pulls?q=is%3Apr+author%3Acharles-r-earp",
            title: "rust-gpu",
            subtitle: "Making Rust a first-class language and ecosystem for GPU shaders.",
            p {
                "Support for ExecutionModes and created an example for compute shaders with wgpu."
            }
            p {
                "Added name to entry point variables for reflection and debugging."
            }
        }
        br {}
        Card {
            href: "https://github.com/vulkano-rs/vulkano/pulls?q=is%3Apr+author%3Acharles-r-earp",
            title: "vulkano",
            subtitle: "Safe and rich Rust wrapper around the Vulkan API.",
            p {
                "Allow customizing the subgroup size on pipeline creation."
            }
            p {
                "Fixed bug in validating the result type of SpecConstantOp."
            }
            p {
                "Reflect push constant requirements by variable usage."
            }
        }
        br {}
        Card {
            href: "https://github.com/qdrant/qdrant/pulls?q=is%3Apr+author%3Acharles-r-earp",
            title: "qdrant",
            subtitle: "Qdrant - High-performance, massive-scale Vector Database for the next generation of AI.",
            p {
                "Feature gate f16 as VectorElementType."
            }
            p {
                "Based on feedback, reduced scope to vector storage, and feature gate f16 as VectorStorageElementType."
            }
        }
        br {}
        Card {
            href: "https://github.com/zed-industries/zed/pulls?q=is%3Apr+author%3Acharles-r-earp",
            title: "zed",
            subtitle: "Code at the speed of thought – Zed is a high-performance, multiplayer code editor from the
            creators of Atom and Tree-sitter.",
            p {
                "Support left tab on linux."
            }
        }
        br {}
        Card {
            href: "https://github.com/starkat99/half-rs/pulls?q=is%3Apr+author%3Acharles-r-earp",
            title: "half",
            subtitle: "Half-precision floating point types f16 and bf16 for Rust. ",
            p {
                "Support for SPIR-V arch."
            }
            p {
                "num-traits implementations."
            }
            p {
                "bytemuck support."
            }
            p {
                "Add AsPrimitive implementations between f16 and bf16."
            }
        }
        br {}
        Card {
            href: "https://github.com/wichtounet/dll/pulls?q=is%3Apr+author%3Acharles-r-earp",
            title: "dll",
            subtitle: "Fast Deep Learning Library (DLL) for C++ (ANNs, CNNs, RBMs, DBNs...)",
            p {
                "Added support for regression prediction, ie raw values instead of classification, as a minor modification of fine_tune_ae()."
            }
        }
        br {}
        Card {
            href: "https://github.com/facebookarchive/caffe2/pulls?q=is%3Apr+author%3Acharles-r-earp",
            title: "caffe2",
            subtitle: "Caffe2 is a lightweight, modular, and scalable deep learning framework.",
            p {
                "Easier Compilation with Python3 (Not accepted)."
            }
            p {
                "Python: add tools for adding gradient descent ops and deploying, saving, and loading nets (Not Accepted)."
            }
        }
    };
    (content, background_image)
}

#[component]
fn Card(
    href: Option<&'static str>,
    title: Option<&'static str>,
    subtitle: Option<&'static str>,
    children: Element,
) -> Element {
    rsx! {
        a {
            class: "hover-border",
            style: "
                display: flex;
                flex-direction: column;
                border-width: 2px;
                border-radius: 10px;
                background-color: var(--background-color);
                color: var(--color);
                padding: 10px;
            ",
            href,
            if let Some(title) = title {
                span {
                    style: "font-size: 2em; font-weight: bold;",
                    {title}
                }
            }
            if let Some(subtitle) = subtitle {
                span {
                    style: "font-style: italic;",
                    {subtitle}
                }
            }
            {children}
        }
    }
}
