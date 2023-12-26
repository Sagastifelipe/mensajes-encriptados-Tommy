use std::io;

use mensajes_tomy::encriptar_mensaje;

const PATH_MENSAJE: &str = "/home/felipe/Desktop/Proyectos/mensajeTomy/mensaje_para_tomy.txt";

fn main() -> io::Result<()>{
    encriptar_mensaje(PATH_MENSAJE.to_string())
}
