pub struct Chess;

enum Colour{
    Light,
    Dark,
    None
}

enum Piece {
    King(Colour),
    Queen(Colour),
    Bishop(Colour),
    Knight(Colour),
    Rook(Colour),
    Pawn(Colour),
    None(Colour),
}

impl std::fmt::Display for Piece{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::King(Colour::Light) => write!(f, "♔"),
            Self::Queen(Colour::Light) => write!(f, "♕"),
            Self::Bishop(Colour::Light) => write!(f, "♗"),
            Self::Knight(Colour::Light) => write!(f, "♘"),
            Self::Rook(Colour::Light) => write!(f, "♖"),
            Self::Pawn(Colour::Light) => write!(f, "♙"),
            Self::King(Colour::Dark) => write!(f, "♚"),
            Self::Queen(Colour::Dark) => write!(f, "♛"),
            Self::Bishop(Colour::Dark) => write!(f, "♝"),
            Self::Knight(Colour::Dark) => write!(f, "♞"),
            Self::Rook(Colour::Dark) => write!(f, "♜"),
            Self::Pawn(Colour::Dark) => write!(f, "♟"),
            _ => write!(f, " "),
        }
    }
}

struct Board([[Piece; 8]; 8]);

impl std::fmt::Display for Board{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0{
            for piece in row{
                print!("{piece}");
            }
            println!();
        }

        Ok(())
    }
}

impl Board{
    pub fn init() -> Self {
        Self([
            [
                Piece::Rook(Colour::Dark), Piece::Knight(Colour::Dark), Piece::Bishop(Colour::Dark), Piece::Queen(Colour::Dark), 
                Piece::King(Colour::Dark), Piece::Bishop(Colour::Dark), Piece::Knight(Colour::Dark), Piece::Rook(Colour::Dark),
            ],
            [
                Piece::Pawn(Colour::Dark), Piece::Pawn(Colour::Dark), Piece::Pawn(Colour::Dark), Piece::Pawn(Colour::Dark), 
                Piece::Pawn(Colour::Dark), Piece::Pawn(Colour::Dark), Piece::Pawn(Colour::Dark), Piece::Pawn(Colour::Dark),
            ],
            [
                Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), 
                Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None),
            ],
            [
                Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), 
                Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None),
            ],
            [
                Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), 
                Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None),
            ],
            [
                Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), 
                Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None), Piece::None(Colour::None),
            ],
            [
                Piece::Pawn(Colour::Light), Piece::Pawn(Colour::Light), Piece::Pawn(Colour::Light), Piece::Pawn(Colour::Light), 
                Piece::Pawn(Colour::Light), Piece::Pawn(Colour::Light), Piece::Pawn(Colour::Light), Piece::Pawn(Colour::Light),
            ],
            [
                Piece::Rook(Colour::Light), Piece::Knight(Colour::Light), Piece::Bishop(Colour::Light), Piece::Queen(Colour::Light), 
                Piece::King(Colour::Light), Piece::Bishop(Colour::Light), Piece::Knight(Colour::Light), Piece::Rook(Colour::Light),
            ],
        ])
    }
}

impl Chess {
    pub fn init(){
        println!("Starting Game...");
        let board = Board::init();
        println!("{board}");
    }
}