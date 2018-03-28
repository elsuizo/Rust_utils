/// ejemplos piolas encontrados por ahi...

//-------------------------------------------------------------------------
// Uno de los types que posee Rust es el "slice" que lo podemos representar
// genericamente &[T] (se lee "slice del type T)
// Este type lo que hace es apuntar automaticamente a el lugar de memoria
// en donde estan los datos de nuestra variable, como ejemplo la siguiente
// funcion imprime los elementos de un Array y un Vector indistintamente
//-------------------------------------------------------------------------


fn print_elements(n: &[f64]) {
    for element in n {
        println!("{:}", element);
    }
}

//-------------------------------------------------------------------------
// podemos con un slice generar rangos por ejemplo:
// print(&vec[0..2]) // imprime los primeros dos elementos
// print(&arr[3..]) // imprime los elemetos comenzando desde 2
// print(&vec[1..3] // imprime los elementos vec[1] vec[2]
//-------------------------------------------------------------------------

//-------------------------------------------------------------------------
// algoritmo para encontrar numeros primos "sieve"
//-------------------------------------------------------------------------
// NOTE(elsuizo:2018-03-25):esto asi no funciona :smile: porque number tiene
// que ser una constante
fn sieve(number: u64) -> Array<bool> {
    let mut result = [true; number];
    for i in 2..number {
        if result[i] {
            let mut j = i * i;
            while j < number {
                result[j] = false;
                j += i;
            }
        }
    }
}

//-------------------------------------------------------------------------
//                      Bloques de codigo
//-------------------------------------------------------------------------
// NOTE(elsuizo:2018-03-26): En Rust los bloques de codigo pueden producir valores
// y tener declaraciones de variables dentro!!!

let msg = {
    // declaracion con let el ; siempre lo necesitamos
    let dandelion_control = puffball.open();
    // expresion + semicolon: llamamos a una funcion y el valor se pierde despues
    // que termine el bloque
    dandelion_control.release_all_seeds(launch_codes);
    // expresion sin ; cuando el metodo es llamado ya que queremos que el valor
    // le sea transferido a la variable msg
    dandelion_control.get_status()
};

// Cualquier bloque de codigo puede tener dentro una declaracion de funcion
fn show_files() -> io::Result<()> {
    let mut v = vec![];
    // ...
    fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
        a.timestamp.cmp(&b.timestamp); // primero comparamos timestamps
        .reverse()
        .then(a.path.cmp(&b.path)) // compare paths to break ties
    }

    // ...
    v.sort_by(cmp_by_timestamp_then_name);
}


// NOTE(elsuizo:2018-03-26):todos los bloques con un if tienen que devolver el mismo
// type sino es un error
let suggested_pet =
    if with_winds {Pet::Buzzard} else {Pet::Hyena};

let favorite_number =
    if user.is_hobbit() {
        "eleventy-one"
    } else {
        9
    }; // error !!! no son del mismo type!!!

// NOTE(elsuizo:2018-03-26):Lo mismo para los match

//-------------------------------------------------------------------------
//                        if/lets
//-------------------------------------------------------------------------
// nos pueden servir para declarar variables de acuerdo a una condicion o
// tambien se usan para extraer el valor de un Result o Option
if let pattern = expr {
    block1
} else {
    block2
}

// ejemplos para sacar de Result o Option valores
if let Some(cookie) = request.session_cookie {
    return restore_session(cookie);
}

if let Err(e) = present_cheesy_anti_robot_task() {
    log_robot_attempt(err);
    politely_acuse_user_of_being_a_robot(); // cuak
} else {
    session.mark_as_humman();
}

// NOTE(elsuizo:2018-03-26):if/let es una abreviacion de match con una sola rama
// o sea que todo lo que puede hacer if/let lo podemos hacer con un match
// o sea:
//
match expresion {
    pattern => {block1},
    _       => {block2}
}

//-------------------------------------------------------------------------
//                        Loops
//-------------------------------------------------------------------------
// Hay cuatro maneras de generar loops
while condition {
    block
}

while let pattern = expr {
    block
}

loop {
    block
}

for pattern in collection {
    block
}


//-------------------------------------------------------------------------
//                        clousere
//-------------------------------------------------------------------------
// los clousures son como funciones-valores que se declaran en linea
// consiste en una lista de argumentos dadas entre barras verticales ||, seguidas
// de una expresion, por ejemplo:

let is_even = |x| x % 2 == 0;

// Rust infiere los types de argumentos y el type de retorno
para llamar a un clousere es como una funcion normal:
is_even(37);

//-------------------------------------------------------------------------
//                        Errores:
// Hay dos tipos de errores: Panics y Results
// Panics: son para ese tipo de errores que no pueden nunca suceder...:
// - Acceder a alementos de un Array en una posicion no permitida
// - Division por zero en enteros
// - Llamando .unwrap() sobre un Option que devuelve None!!!
// - Un assert() que falla!!!
// Si una de estas ocurre es por culpa del programador, por ello se dice que
// como regla general no debemos entrar en panic
//-------------------------------------------------------------------------
// Podemos definir alias para los types en general y para un Result en particular:
pub type Result<T> = result::Result<T, Error>;

// esto lo que hace es que cuando llamemos a Result<String> == Result<String, Error> (para el
// compilador
// Todos los errores de la libreria estandar definen types de errores comunes, como:
// std::io::Err std::fmt::Error, std::str::Utf8Error ...
// Todos ellos implementan el Trait std::error::Error, lo que significa que comparten el mismo
// comportamiento con las siguientes caracteristicas:
// - Todos ellos son printables con println!("message{:}", err);
// - err.description() : Return a &str
// - err.cause(): return un Option<&Error> el error subyacente
// Una funcion piola para imprimir toda la informacion de un error
//
fn print_error(mut err: &Error) {
    let _ = writeln!(stderr(), "error: {:}", err);
    while let Some(cause) = err.cause() {
        let _ = writeln!(stderr(), "caused by: {:}", cause);
        err = cause;
    }
}

// Cuando no queremos tratar los posibles errores que puedan suceder, podemos pasarle la pelota a
// el que llama a la funcion que estamos haciendo o evaluando, asi propagamos el error a un nivel
// mas alto de llamada(que eventualmente) El que realiza esto por nosotros es el operador ?
// Solo tenemos que llamar al operador para toda aquella funcion que devuelva Resul<>
// ejemplo:
let weather = get_weather(hometown)?;

// El comportamiento de ? depende si la funcion retorna un resultado exitoso o un error
// - Sobre un resultado positivo unwrap el Result para obtener el valor que lleva adentro.
// - Sobre un Error imediatamente retorna a la funcion que la llamo pasando el error a la funcion
// que la llama.
// Algunas veces aparece en todas las lineas de codigo ejejej
//
use std::fs;
use std::io;
use std::path::Path;

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? { // open a dir maybe fails...
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file())?; // renaming could fails...
    }
    Ok(()) // phew!!!
}

// podemos tener muchas fuentes de errores y puede pasar que no haya una conversion de un error a
// std::io::Err. Por ejemplo el crate image define su propio error ImageError y la conversion a el
// error estandar std::io::Error
// Una manera de evitar esto es convirtiendo al type Box<std::erorr::Error> el cual representa
// cualquier error. Por eso podemos definir el siguiente alias para tratar con estos errores:
use std::io::{self, BufRead};

// Read integers from a text file
// The file should have one number on each line
fn read_numbers(file: &mut BufRead) -> Result<Vec<i64>, io::Error>  {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?; // reading lines can fail!!!
        number.push(line.parse()?); // parsing can fails!!!
    }
    Ok(numbers)
}


type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

// con estas definiciones el type de retorno en `read_numbers()` Result<Vec<i64>>
// y la funciones que tengan ? automaticamente las convertira e el Result grande
//-------------------------------------------------------------------------
//                        errores propios
//-------------------------------------------------------------------------
// Supongamos que hacemos una libreria para leer .json y necesitamos un type de error propio cuando
// no se pudo obtener la informacion. Podriamos hacer:
//
// json/src/errors.rs
#[Derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

// esta estructura se llamara desde json::error::JsonError
// y cuando tengamos un error proveniente de este type podemos hacer:
return Err(JsonError {
    message: "Expectd ] at end of array ".to_string(),
    line: current_line,
    column: current_column
});

// Para que funcione correctamente necesitamos implementar algunos traits
//
use std;
use std::fmt;

// Errors should be printables!!!
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt:Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// Errors should implement the std::error::Error trait
//
//-------------------------------------------------------------------------
//                        Modulos
//-------------------------------------------------------------------------
//Los modulos en Rust son namespaces, osea espacio donde agrupar funciones, types, constantes y
//demas...
// Cuando trabajamos en un archivo solo y queremos un nuevo namespace(modulo) podemos hacerlo de la
// siguiente manera:
mod spores {
    use cells::Cell;
    /// documentacion clara para la estructura que viene
    pub struct Spore {
        ///...
    }

    /// Simulate the production of a spore by meiosis
    pub fn produce_spore(factory: &mut Sporangium) -> Spore {
        ///...
    }
    /// Mix genes to prepare for meiosis (part of interphase)
    fn recombine(parent: &mut Cell) {
        ///...
    }

    ///...
}

// Los modulos pueden estar anidados, es muy comun ver la siguiente estructura:

mod plant_structures {
    pub mod roots {
        //...
    }

    pub mod stens {
        //...
    }

    pub mod leaves {
        //...
    }
}

// o sea una raiz que no es publica y los nodos si son publicos
// Pero si es mucho codigo puede ser un lio..., por ello podemos poner cada modulo en un archivo
// diferente y llamarlo con:
mod lalala;
// Podemos ponerlo en una carpeta llamada lalala y que adentro tenga el archivo mod.rs
// O podemos ponerlo directamente en un archivo que se llame lalala.rs


//-------------------------------------------------------------------------
//                        structs
//-------------------------------------------------------------------------
// Ademas de las estructuras normales existen tambien las estructuras llamadas
// tuple-like struct porque parecen a una tupla
struct Bounds(usize, usize);

// y constuimos un objeto de este tipo:
let image_bound = Bounds(1024, 768);
// luego accedemos a los valores como si fuera una tupla:
assert_eq!(image_bound.0 * image_bound.1, 786432);
// Tambien podemos hacer que los elementos sean publicos:
pub struct Bounds(pub usize, pub usize);

// NOTE(elsuizo:2018-03-27): Estas estructuras son utilizadas cuando tenemos un type nuevo
// cuando tenemos una estructura con un solo componente que queremos que se chequee su type de
// manera rigurosa. Por ejemplo si estamos trabajando con un texto de caracteres ASCII solamente,
// una buena idea seria definir una type nuevo asi:
struct Ascii(Vec<u8>);
// Vemos que es mucho mas entedible que estamos trabajando con un texto Ascii que con un vector de
// chars
//

// NOTE(elsuizo:2018-03-27): Otra estructura que podemos declarar es la que se denomina:
// "unit-like struct" que es basicamente una declaracion de una estructura sin elementos
//
struct Onesuch;

// Un valor de este type no ocupa nada de memoria, como el type unidad ()
//
//-------------------------------------------------------------------------
//                        structs en memoria
//-------------------------------------------------------------------------
// Si tenemos una estructura de la siguiente manera:
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}
/*
                                            Image
            +---+---------------+---+---+---+--+---+---+----+---+--------------------+
 stack frame|   |               |   |   |   |  |   |   |1024|768|                    |
            |   |               |   |   |   |  |   |   |    |   |                    |
            +---+---------------+-+-+---+---+--+---+---+----+---+--------------------+
                       |           pixels                 size
                       |
                       |
 +------+----+---+---+-V-+---+---+---+---+---+---+---+---+---+---+-------+
 |      | ...| 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |...|       |
 |      |    |   |   |   |   |   |   |   |   |   |   |   |   |   |       |
 +------+----+---+---+---+---+---+---+---+---+---+---+---+---+---+-------+
 */

//-------------------------------------------------------------------------
//                 definiendo metodos de una estructura
//-------------------------------------------------------------------------
// para definir un metodo nuevo tenemos que utilizar un bloque impl nuevo
// ejemplo:
//
pub struct Queue {
    older: Vec<char>, // older elements, eldest last
    younger: Vec<char> // younger elements, yougest last
}

impl Queue {
    // push a character onto the back of a queue
    pub fn push(&mut self, c: char) {
        self.younger.push(c); // notar que push es un metodo de Vec
    }
    // pop a character off the front of a queue. Return `Some(c)` if there
    // was a character to pop, or `None` if the queue was empty
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            // Bring the elements in younger over to older, and put them in the promised order
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        // aca ya el orden esta garantizado asi que podemos utilizar el metodo pop de Vec
        self.older.pop() // que retorna un Option como queremos
    }

}
// entonces podemos escribir
//
let mut q = Queue {older: Vec::new(), younger: Vec::new()};
q.push('0');
q.push('1');
assert_eq!(q.pop(), Some('0'));

q.push('i')
assert_eq!(q.pop(), Some('1'));
assert_eq!(q.pop(), Some('i'));
assert_eq!(q.pop(), None);

//-------------------------------------------------------------------------
//                        estructuras genericas
//-------------------------------------------------------------------------
// en el codigo anterior nuestra queue solo aceptaba valores que sean chars
// si queremos el mismo algoritmo pero que acepte otro type tendremos que escribir todo de nuevo
// pero reemplazando el type.
// Por ejemplo para el caso anterior, el caso generico seria:
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}
// Que como con los otros lenguages que permiten programacion generica, lo ultimo se lee: "para
// cualquier elemento de type T"
// y la implementacion generica seria:
//
impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {older: Vec::new(), younger: Vec::new()} // no hace falta el parametro en los Vec!!!
    }
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    //...
}

// podemos leer el impl<T> como: "para cualquier type T, estos son los metodos disponibles sobre la
// estructura Queue<T>". Entonces podemos usar los parametros de type T como un type en la
// definicion de los metodos(por ejemplo en push)
// // NOTE(elsuizo:2018-03-27):Yo pense que habia que poner el parametro T en todos los
// metodos !!! por eso era que me daba error antes
//
//-------------------------------------------------------------------------
//                  structs con lifetime parameters
//-------------------------------------------------------------------------
// Como vimos cuando tenemos una estructura que tiene elementos con punteros(o referencias mejor
// dicho) debemos especificar su tiempo de vida con los "lifetime parameters" como ejemplo:
/// Estructura que guarda referencias a los elementos mas chicos y mas grandes de un slice
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}
// aca el parametro de tiempo de vida seria elt (podemos ponerle cualquier nombre). Dicho
// coloquialmente seria: "Dado un tiempo de vida 'elt, podemos hacer una estructura Extrema<'elt>
// que guarde una referencia con ese tiempo de vida"
// Aca ponemos una funcion que recorre todo el slice y busca el maximo y el minimo valor
fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {least = &slice[i];}
        if slice[i] > *greatest {greatest = &slice[i];}
    }
    Extrema {greatest, least}
}

// notemos que para llamar a la funcion no necesitamos poner el parametro de lifetime porque Rust
// lo infiere automagicamente
// un ejemplo de uso de esta funcion seria:
let a = [0, -3.0, 1, 15, 48];
let e = find_extrema(&a);
assert_eq!(*e.least, -3);
assert_eq!(*e.greatest, 48);

/// Como los el type de retorno utiliza el misom lifetime que la funcoion que lo llama no es
/// necesario que pongamos explicitamente los parametros, osea que hubiera sido lo mismo:
fn find_extrema(slice: &[i32]) -> Extrema {
    //...
}

//-------------------------------------------------------------------------
//                        mutabilidad interior
//-------------------------------------------------------------------------
// cuando queremos que un elemento de una estructura sea mutable tenemos que tener cuidado, por
// ejemplo supongamos que tenemos un Robot y cuyo sistema de control tiene una estructura central e
// importante(que no se puede modificar)
pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    leg_device: [fd::FileDesc; 8],
    //...
}

// cuando el Robot se pone en marcha se establece esta estructura y los valores nunca cambian. Cada
// sistema principal del Robot es manejado por una diferente estructura y cada una tiene un puntero
// al Robot
//
use std::rc::Rc;

pub struct SpiderSensors {
    robot: Rc<SpiderRobot>, // <--- Pointer to settings I/O
    eyes: [Camera; 32];
    motion: Accelerometer,
    //...
}

// Donde Rc<> es un "smart pointer" que es la abreviatura de Reference counting
// Pero Rc<> es un box que es compartido y por ello es inmutable!!!. Que pasa si queremos hacer un
// sistema de logging a la estructura del Robot usando los metodos estandar de File I/O aqui se
// presenta el problema que los Files son siempre mut, osea lo que necesitamos es una estructura
// mutable(el File) dentro de una inmutable(el Robot), esto es comun que suceda y se conoce como
// mutabilidad interior. Rust ofrece muchas alternativas para esto aca vamos a ver dos:
// Cell<T> y RefCell<T>, los dos en el modulo std::cell
// Cell<T> es una estructura que contiene un unico valor privado del type T. Lo que lo diferencia
// de las otras estructuras es que podemos accder y cambiar ese campo aunque no tengamos acceso a
// la Cell en si misma.
// - Cell::new(value) crea una cell moviendo el valor dado dentro de el.
// - cell.get() retorna una copia del valor en la cell
// - cell.set(value) guarda el valor dado en la cell, descartando el valor previo
//
// Entonces podemos agregar a nuestro Robot un contador de errores:
use std::cell::Cell;

pub struct SpiderRobot {
    //...
    hardware_errors_counter: Cell<u32>,
    //..
}

// y entonces cualquiera de los metodos de SpiderRobot pueden acceder a al u32 usando el .get() y
// el .set()
impl SpiderRobot {
    // incrementa el error en 1
    pub fn add_hardware_error(&self) {
        let n = self.hardware_errors_counter.get();
        self.hardware_errors_counter.set(n + 1);
    }
    // verdadero si cualquier error de hardware ha sido reportado
    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_errors_counter.get() > 0
    }
}

// pero esto no resuelve nuestro problema de logging, Cell no deja llamar metodos mut sobre valores
// compartidos. El metodo .get() retorna una copia del valor en la cell, por eso solo funciona si T
// implementa el trait Copy. Para logging necesitamos un File mut y el type File no implementa
// Copy. Entonces la herramienta que debemos utilizar es RefCell<T>.
// RefCell<T>: Es un type generico que contiene un unico valor de type T, a diferencia de Cell<T>
// soporta el prestamo de referencia de ese valor de type T
// RefCell::new(value): crea una nueva RefCell moviendo un valor dentro de el.
// ref_cell.borrow(): retorna una Ref<T>, la cual es esencialmente solo una referencia compartida
// al valor guardado en ref_cell
// Este metodo panics si el valor es prestado mutablemente
// ref_cell.borrow_mut(): Retorna una RefMut<T>, esecialmente una referencia mutable al valor en
// ref_cell. Este metodo panics si el valor ya ha sido prestado

// Poniendo esto en nuestro Robot:
pub struct SpiderRobot {
    //...
    log_file: RefCell<File>,
    //...
}

impl SpiderRobot {
    // escribe una linea en el archivo de log
    pub fn log(&self, message: &str) {
        let mut file = self.log_file.borrow_mut();
        writeln!(file, "{}", message).unwrap();
    }
}
