use anyhow::bail;
use anyhow::Result;
use strict_yaml_rust::StrictYaml;

#[derive(Debug)]
pub struct AirportList {
    //content: HashSet<Airport>,
    content: Vec<Airport>,
}

#[derive(Debug)]
struct Airport {
    iata: String,
    icao: String,
    name: String,
}

impl Airport {
    fn new(iata: &str, icao: &str, name: &str) -> Self {
        Airport {
            iata: iata.to_string(),
            icao: icao.to_string(),
            name: name.to_string(),
        }
    }
}

/// Todo
/// error if same icao or same name
impl AirportList {
    pub fn new() -> Self {
        AirportList {
            content: Vec::new(),
        }
    }
    /// Add an airport performs checks
    /// and returns best name
    pub fn add(
        &mut self,
        iata: &StrictYaml,
        icao: &StrictYaml,
        name: &StrictYaml,
    ) -> Result<String> {
        let iata = iata.as_str();
        let icao = icao.as_str();
        let name = name.as_str();
        //
        match (iata, icao, name) {
            // Case 1 iata and icao and name are defined
            (Some(iata), Some(icao), Some(name)) => {
                if self.has_iata(iata) && iata != "???" {
                    bail!("iata:{} already in Airport List.", iata);
                } else if self.has_icao(icao) && icao != "????" {
                    bail!("icao:{} already in Airport List.", icao);
                } else if self.has_name(name) {
                    bail!("name:{} already in Airport List.", name);
                } else {
                    self.content.push(Airport::new(iata, icao, name));
                    Ok(best_name(iata, icao, name))
                }
            }
            // Case 2
            (Some(iata), None, None) => {
                if self.has_iata(iata) {
                    Ok(iata.to_string())
                } else {
                    bail!("iata {} not in database", iata);
                }
            }
            // Case 3
            (None, Some(icao), None) => {
                if self.has_icao(icao) {
                    Ok(icao.to_string())
                } else {
                    bail!("icao {} not in database", icao);
                }
            }
            // Case 4
            (None, None, Some(name)) => {
                if self.has_name(name) {
                    Ok(name.to_string())
                } else {
                    bail!("name {} not in database", name);
                }
            }
            // Case 5
            (None, None, None) => {
                bail!("No airport name, IATA or ICAO.");
            }
            // All  other cases
            _ => {
                bail!(
                    "Badly defined airport iata={:?}, icao={:?}, name={:?}",
                    iata,
                    icao,
                    name
                );
            }
        } // match
    }

    fn has_iata(&self, iata: &str) -> bool {
        for airport in self.content.iter() {
            if iata == airport.iata {
                return true;
            }
        }
        false
    }

    fn has_icao(&self, icao: &str) -> bool {
        for airport in self.content.iter() {
            if icao == airport.icao {
                return true;
            }
        }
        false
    }

    fn has_name(&self, name: &str) -> bool {
        for airport in self.content.iter() {
            if name == airport.name {
                return true;
            }
        }
        false
    }
}

fn best_name(iata: &str, icao: &str, name: &str) -> String {
    if iata != "???" {
        iata.to_string()
    } else if icao != "????" {
        icao.to_string()
    } else {
        name.to_string()
    }
}
