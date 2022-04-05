#[derive(PartialEq, Clone)]
pub struct PokemonNumber(u16);
pub struct PokemonName(String);
pub struct PokemonTypes(Vec<PokemonType>);

enum PokemonType {
    Electric,
    Fire,
}

pub struct Pokemon {
    pub number: PokemonNumber,
    name: PokemonName,
    types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
        Self {
            number,
            name,
            types,
        }
    }
}

impl TryFrom<u16> for PokemonNumber {
    type Error = ();

    fn try_from(number: u16) -> Result<Self, Self::Error> {
      if number > 0 && number < 899 {
        Ok(Self(number))
      } else {
        Err(())
      }
    }
}

impl From<PokemonNumber> for u16 {
    fn from(number: PokemonNumber) -> u16 {
      number.0
    }
}

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.is_empty() {
            Err(())
        } else {
            Ok(Self(name))
        }
    }
}

impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(types_string: Vec<String>) -> Result<Self, Self::Error> {
        if types_string.is_empty() {
            Err(())
        } else {
            let mut pokemon_types_string = vec![];

            for a_type in types_string.iter() {
                match PokemonType::try_from(String::from(a_type)) {
                    Ok(pokemon_type) => pokemon_types_string.push(pokemon_type),
                    _ => return Err(())
                }
            }

            Ok(Self(pokemon_types_string))
        }
    }
}

impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(a_type: String) -> Result<Self, Self::Error> {
        match a_type.as_str() {
            "Electric" => Ok(Self::Electric),
            "Fire" => Ok(Self::Fire),
            _ => Err(()),
        }
    }
}
