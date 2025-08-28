use std::env;
mod audio;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 1 {
        println!("Uso: {} <caminho_do_arquivo_wav>", args[0]);
        println!("Exemplo: {} audio.wav", args[0]);
        return;
    }
    
    let wav_path = &args[1];

    let wav_data = audio::read::WavData::from_file(wav_path).unwrap();
    println!("{}", wav_data.get_info());
}
