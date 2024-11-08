use std::{
    fs::File,
    io::{self, BufReader},
    os::windows::process::CommandExt,
    process::Command,
};

use rodio::{Decoder, OutputStream, Sink};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn to_audio(text: &str) -> String {
    let sentences = split_by_stop_words(text);
    println!("Converting text to sentences: {}", sentences.join(", "));

    // 出力デバイスとストリームの取得
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let output = "output.wav";
    for sentence in sentences {
        let _ = speech(&sentence, output);

        // play audio
        // 音声ファイルを再生
        let file = BufReader::new(File::open(output).unwrap());
        let source = Decoder::new(file).unwrap();

        // シンクを作成して音声を再生
        sink.append(source);
        sink.sleep_until_end(); // 再生が終わるまで待機
    }

    "".to_string()
}

// コマンドで音声ファイルを作成し保存する
fn speech(text: &str, output: &str) -> Result<(), io::Error> {
    let lang = "ja-JP";
    let voice = "ja-JP-Wavenet-C";

    let mut command = Command::new("speech.exe");

    #[cfg(target_os = "windows")]
    command.creation_flags(0x08000000);

    let status = command
        .args([
            "--output", output, "--lang", lang, "--voice", voice, "--text", text,
        ])
        .status()?;

    if !status.success() {
        eprintln!(
            "Error: Failed to generate blank video - status: {:?}",
            status
        );
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to generate blank video",
        ));
    }

    Ok(())
}

fn split_by_stop_words(text: &str) -> Vec<String> {
    static STOP_WORDS: &[char] = &['。', '、', '？', '！', '…', ',', '.', '?', '!'];

    let mut result = Vec::new();
    let mut start_byte = 0;

    // Convert text to char indices iterator which gives both byte position and character
    let char_indices: Vec<(usize, char)> = text.char_indices().collect();

    for (i, &(_, c)) in char_indices.iter().enumerate() {
        // Handle the last character
        if i == char_indices.len() - 1 {
            result.push(text[start_byte..].to_string());
            break;
        }

        if STOP_WORDS.contains(&c) {
            let next_byte = if i + 1 < char_indices.len() {
                char_indices[i + 1].0
            } else {
                text.len()
            };

            // Push the text including the stop word
            result.push(text[start_byte..next_byte].to_string());
            start_byte = next_byte;
        }
    }

    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![to_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
