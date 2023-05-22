#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    // single character tokens
    Minus, // -
    Plus, // +
    SamaDengan, // =
    KurungBuka, // (
    KurungTutup, // )
    KurawalBuka, // {
    KurawalTutup, // }
    ImporBuka, // [
    ImporTutup, // ]
    Koma, // ,
    Titik, // .
    Bagi, // /
    Bintang, // *
    Tanya, // ?
    Petik, // "
    Char, // '
    Pindah, // ~

    // one or two char tokens
    Seru, // !
    SeruSama, // !=
    Sama, // ==
    Kurang, // <
    KuranSama, // <=
    Lebih, // >
    LebihSama, // >=
    Konektor, // ::
    ImporLokal, // []
    Comment, // //

    // reserved tokens
    Buat, // let 
    Tulis, // print
    Atau, // or
    Dan, // and
    Jika, // if
    Lainnya, // else

    // types
    Catat, // string
    Angka, // int
    Desimal, // float
    Karakter, // char
    Boolean, // bool

    // eof
    Eof, // ...
    Eow, // space
    Eol, // ;
}