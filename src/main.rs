use base64::encode;
use yew::prelude::*;
use web_sys::{HtmlInputElement, FileList};
use gloo::file::{File, callbacks};

#[derive(Properties, PartialEq, Clone)]
pub struct Upload{
    pub file: Vec<File>,
}

#[derive(Properties, PartialEq)]
pub struct FileInfoProps{
    file: File,
}

#[function_component(FilesInfo)]
fn files_info(props: &Upload) -> Html{
    props
    .file.iter().map(|v| html!{<FileInfo file={v.clone()} />}).collect()
}

#[function_component(FileInfo)]
fn file_info(FileInfoProps {file}: &FileInfoProps) -> Html{
    let data_state = use_state_eq(||vec![]); // state for image bytes.

    let data_state_outer = data_state.clone();
    // the reader should last until the file is read
    let _reader = use_memo( 
        |file| {
            callbacks::read_as_bytes(file, move |result| {
                data_state_outer.set(result.unwrap());
            })
        },
        file.clone(),
    );

       
    html!{
        <>
         <img src={format!("data:{};base64,{}", file.raw_mime_type(), encode(&(*data_state)))} />
        </>
    }
}

#[function_component(App)]
fn app() -> Html {

    let input_state = use_state_eq(|| vec![]);

    let input_state_outer = input_state.clone();

    let input_ref = NodeRef::default();
    let input_ref_outer = input_ref.clone();
    let onchange = Callback::from(move |_:Event|{
        let input: HtmlInputElement = input_ref.cast::<HtmlInputElement>().unwrap();
        let files:Option<FileList> = input.files();

        if let Some(file) = files {
             let files:Vec<File> = js_sys::try_iter(&file)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from)
                .collect();

            input_state_outer.set(files);
        }
    });


    html! { 
        <>
          <FilesInfo file = {(*input_state).clone()}/>
          <input type = "file" accept = "image/*" ref = {input_ref_outer.clone()} {onchange} multiple = {true}/>
        </>
     }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
