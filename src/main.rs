use std::{io::{Read, Write}, fs, fs::File};

#[derive(Clone, Debug)]
struct Chunk {
    bank : u8,
    data : Vec<u8>,
    name : String,
}

fn build_chunk(c_bank: u8, c_data: &Vec<u8>, c_name: String) -> Chunk {

    // .clone() just to be sure

    Chunk{
        bank : c_bank.clone(),
        data : c_data.clone(),
        name : c_name.clone(),
    }
}

fn deconstruct_tic(path: String) -> Vec<Chunk> {
    // reading the .tic file
    let mut f = File::open(String::from(path.clone())).expect("No file found!");

    // get file size
    let size : u64 = fs::metadata(path.clone()).expect("No file found").len();

    // creating a vector to store the bytes
    let mut buf = vec![0; size as usize];

    // put the bytes into the vector
    let _ = f.read_exact(&mut buf);

    // TODO :
    // separate chunks

    let mut chunks : Vec<Chunk> = vec![];
    let mut check = 0;

    // static types are good

    let mut chunk_size : u16 = 0;
    let mut chunk_bank : u8 = 0;
    let mut chunk_type : &str = "";
    let mut chunk_data : Vec<u8> = vec![];

    for i in buf {

        // chunks follow the scheme of
        // type(5 bits) + bank(3 bits)

        chunk_type = match check {
            0 => match i & 0b00011111 {
                1 => "Tiles",
                2 => "Sprites",
                4 => "Map",
                5 => "Code",
                6 => "Flags",
                9 => "Samples",
                10 => "Waveform",
                12 => "Palette",
                14 => "Music",
                15 => "Patterns",
                17 => "Default",
                18 => "Screen",
                19 => "Binary",
                _ => "(Reserved)"
                },
            _ => chunk_type,
        };
        chunk_bank = match check {
            0 => i & 0b11100000,
            _ => chunk_bank,
        };

        // size(16 bits)

        chunk_size = match check {
            1 => i as u16,
            2 => chunk_size + ((i as u16) << 8),
            _ => chunk_size,
        };

        // reserved(8 bits)

        // actual data(size bits)

        if check == 4 {
            if chunk_size > 0 {
                chunk_size -= 1;
                chunk_data.push(i);
            } else {
                check = 0;
            }
        }

        // handle data insertion

        if check < 3 {
            // cycle state

            check += 1;
        } else if chunk_size == 0 {
            // reset state

            check = 0;

            // add chunk

            chunks.push(
                build_chunk(
                    chunk_bank,
                    &chunk_data,
                    chunk_type.into()
                )
            );
            chunk_data.clear();
        } else {
            // set state

            check = 4;
        }
    }

    chunks
}

fn extract(from: Vec<Chunk>, name: String) -> Chunk {
    for i in from {
        if i.name == name {
            return i
        }
    }

    return Chunk{
        bank : 0,
        data : vec![],
        name : name,
    }
}

fn replace(from: Vec<Chunk>, what: Chunk) -> Vec<Chunk> {
    let mut new : Vec<Chunk> = vec![];

    let mut added : bool = false;

    for i in from {
        if i.name == what.name {
            new.push(what.clone());
            added = true;
        } else {
            new.push(i);
        }
    }

    if !added {
        new.push(what.clone());
    }

    new
}

fn find(from: Vec<Chunk>, name: String) -> bool {
    for i in from {
        if i.name == name {
            return true
        }
    }

    false
}

fn flatten(thick : Vec<Vec<u8>>) -> Vec<u8> {
    let mut new : Vec<u8> = vec![];

    for i in thick {
        for k in i {
            new.push(k);
        }
    }

    new
}

fn compress(wide : Vec<u8>) -> Vec<u8> {
    let mut now : u8 = 0;
    let mut chn : i32 = 0;

    let mut new : Vec<u8> = vec![];

    for i in wide {
        if chn == 0 {
            now = i.into();
        } else {
            now += i << 4;
            new.push(now);
            now = 0;
        }
        chn = 1 - chn;
    }

    new
}

fn expand(from: Vec<(u8, u8, u8)>) -> Vec<u8> {
    let mut new : Vec<u8> = vec![];

    for i in from {
        new.push(i.0);
        new.push(i.1);
        new.push(i.2);
    }

    new
}

fn construct_tic(path: String, from: Vec<Chunk>) -> () {
    println!("{}", path);

    let mut file = fs::OpenOptions::new().create(true).write(true).open(path).expect("No");

    for i in &from {
        let type_id = match i.name.as_str() {
            "Tiles" => 1,
            "Sprites" => 2,
            "Map" => 4,
            "Code" => 5,
            "Flags" => 6,
            "Samples" => 9,
            "Waveform" => 10,
            "Palette" => 12,
            "Music" => 14,
            "Patterns" => 15,
            "Default" => 17,
            "Screen" => 18,
            "Binary" => 19,
            _ => 32,
        } & 0b00011111 ;

        println!("{}", type_id);

        let size = i.data.len() as u16;

        println!("{}", size);

        let size_low : u8 = (size & 0b0000000011111111) as u8;
        let size_high : u8 = ((size & 0b1111111100000000) >> 8) as u8;

        println!("{} {}", size_low, size_high);

        let bank = i.bank >> 5;

        println!("{}", bank);

        let mut chunk_bytes : Vec<u8> = vec![];

        println!("{}", bank + type_id);

        chunk_bytes.push(bank + type_id);
        chunk_bytes.push(size_low);
        chunk_bytes.push(size_high);
        chunk_bytes.push(0);

        for k in i.data.clone() {
            chunk_bytes.push(k);
        }

        let _ = file.write_all(&chunk_bytes);
    }
}

fn main() -> () {
}
