
enum Publication {
    Book(Book),
    Magazine(Magazine)
}
struct Book {
    title:String,
    author:String,
    page_count:u16
}
struct Magazine {
    title:String,
    issue:u16,
    topic:String
}
fn print_publication (publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(ref book)=>{ println!("name:{},author:{},page_count:{}:", book.title,book.author,book.page_count);}
            Publication::Magazine(ref magazine)=> {println!("name:{},issue:{},topic:{}",magazine.title,magazine.issue,magazine.topic);}
        }
    }
}

fn main() {
    let book1 = Book {
        title: "Beginning Rust ".to_string(), //to.string() ifadesi string slice türünü stringe çevirir.Compiler bu girilen cümleyi string slice olarak algılar en başta strcut tanımlarken string şeklinde tanımladığımız için string slice ifadeleri stringe çevirdik.
        author:"Carlo Milanesi".to_string(),
        page_count:400
    };
    let magazine1 = Magazine {
        title:"Rust Magazine".to_string(),
        issue:30,
        topic:"Rust programming techniques".to_string()
    };
    let publications = vec![Publication::Magazine(magazine1),Publication::Book(book1)];

    print_publication(publications);

}

