use std::fs::File;
use std::io::Read;

use gen_welcome::generate;

fn main() {
    let mut args = std::env::args().skip(1);

    let Some(background) = args.next() else {
        panic!("El background es necesario")
    };
    let Some(avatar) = args.next() else {
        panic!("El avatar es necesario")
    };
    let Some(name) = args.next() else {
        panic!("El nombre es necesario")
    };
    let Some(members) = args.next().map(|m| {
        m.parse::<u64>()
            .expect("No se pudo parsear la cantidad de miembros")
    }) else {
        panic!("La cantidad de miembros es necesario")
    };
    let Some(out) = args.next() else {
        panic!("El out file es necesario")
    };

    let avatar = {
        let mut f = File::open(avatar).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("buffer overflow");

        buffer
    };

    generate(
        &background,
        &avatar,
        &name,
        members,
        include_bytes!("../../../static/fonts/WorkSans-Bold.ttf"),
        include_bytes!("../../../static/fonts/WorkSans-Regular.ttf"),
        &out,
    )
    .unwrap();
}
