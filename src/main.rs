use std::fmt::{Display, Error, Formatter};

use clap::Parser;

use nom_bibtex::{Bibliography, Bibtex};

#[derive(Default)]
struct Pages {
    from: Option<i64>,
    to: Option<i64>,
}

impl Pages {
    fn from_string(s: &String) -> Self {
        let mut pages = s
            .split(|c: char| !c.is_ascii_digit())
            .filter_map(|i| i.parse::<i64>().ok());
        let from = pages.next();
        let to = pages.next();
        Pages { from, to }
    }
}

impl Display for Pages {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let to_str: String = self.to.map(|p| p.to_string()).unwrap_or("".to_string());
        let from_str: String = self.from.map(|p| p.to_string()).unwrap_or("".to_string());
        write!(f, "page:\n  from: {}\n  to: {}\n", from_str, to_str)
    }
}

#[derive(Debug)]
struct Series {
    series: Option<i64>,
}

impl From<Option<&String>> for Series {
    fn from(s: Option<&String>) -> Series {
        Series {
            series: s.map_or(None, |i| i.parse::<i64>().ok()),
        }
    }
}

impl Display for Series {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "series: {}\n",
            self.series
                .clone()
                .map(|i| i.to_string())
                .unwrap_or("".to_string())
        )
    }
}

struct Authors {
    authors: Vec<String>,
}

impl Authors {
    fn from_string(s: &String) -> Self {
        let s = s.replace("\n", " ");
        let authors = s
            .split(" and ")
            .map(str::to_string)
            .map(|s| {
                s.split(',')
                    .rev()
                    .fold("".to_string(), |mut acc, s| {
                        acc.push(' ');
                        acc.push_str(s);
                        acc
                    })
                    .trim()
                    .to_string()
            })
            .collect();
        Authors { authors }
    }
}

impl Display for Authors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "authors:\n")?;
        for a in &self.authors {
            write!(f, "  - \"{}\"\n", a)?;
        }
        Ok(())
    }
}

struct Volume {
    nb: Option<i64>,
}

impl From<Option<&String>> for Volume {
    fn from(s: Option<&String>) -> Self {
        Volume {
            nb: s.map(|s| s.parse::<i64>().ok()).flatten(),
        }
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "volume: ")?;
        match self.nb {
            Some(n) => write!(f, "{}\n", n),
            None => write!(f, "\n"),
        }
    }
}

struct Doi {
    s: String,
}

impl From<Option<&String>> for Doi {
    fn from(s: Option<&String>) -> Doi {
        Doi {
            s: s.map(|s| s.clone()).unwrap_or(String::new()),
        }
    }
}

impl Display for Doi {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "doi: \"{}\"\n", self.s)
    }
}

struct Year {
    year: i64,
}

impl From<&str> for Year {
    fn from(s: &str) -> Year {
        Year {
            year: s.parse::<i64>().unwrap(),
        }
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "year: {}\n", self.year)
    }
}

struct Title {
    title: String,
}

impl From<&String> for Title {
    fn from(s: &String) -> Title {
        Title { title: s.clone() }
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "title: \"{}\"\n", self.title)
    }
}

struct Abstract {
    abs: String,
}

impl From<&String> for Abstract {
    fn from(s: &String) -> Abstract {
        Abstract { abs: s.clone() }
    }
}

impl Display for Abstract {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}\n", self.abs)
    }
}

enum Place {
    Journal(Name),
    Conference(Name),
}

impl Display for Place {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Place::Journal(n) => write!(f, "journal:\n  name: \"{}\"\n  shortname: \"\"\n", n),
            Place::Conference(n) => {
                write!(f, "conference:\n  name: \"{}\"\n  shortname: \"\"\n", n)
            }
        }
    }
}

struct Name {
    name: String,
}

impl From<&String> for Name {
    fn from(s: &String) -> Name {
        Name { name: s.clone() }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.name)
    }
}

struct Url {
    link: String,
}

impl From<&String> for Url {
    fn from(s: &String) -> Url {
        Url { link: s.clone() }
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "www: \"{}\"\n", self.link)
    }
}

struct Publisher {
    publi: Option<String>,
}

impl From<Option<&String>> for Publisher {
    fn from(s: Option<&String>) -> Publisher {
        Publisher {
            publi: s.map(String::from),
        }
    }
}

impl Display for Publisher {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "publisher: ")?;
        match &self.publi {
            Some(n) => write!(f, "\"{}\"\n", n),
            None => write!(f, "\n"),
        }
    }
}

struct Paper {
    auth: Authors,
    pages: Pages,
    vol: Volume,
    year: Year,
    doi: Doi,
    title: Title,
    place: Place,
    url: Url,
    abs: Abstract,
    series: Series,
    publi: Publisher,
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "---\n")?;
        write!(f, "{}", self.auth)?;
        write!(f, "{}", self.pages)?;
        write!(f, "{}", self.vol)?;
        write!(f, "{}", self.series)?;
        write!(f, "{}", self.place)?;
        write!(f, "{}", self.title)?;
        write!(f, "{}", self.publi)?;
        write!(f, "{}", self.year)?;
        write!(f, "{}", self.doi)?;
        write!(f, "{}", self.url)?; // Not accepted by hugo
        write!(f, "---\n")?;
        write!(f, "{}\n", self.abs)
    }
}

impl From<&Bibliography> for Paper {
    fn from(b: &Bibliography) -> Self {
        let tags = b.tags();
        //println!("{:#?}", tags);
        let conf = tags
            .get("booktitle")
            .map(|j| Place::Conference(Name::from(j)));
        let place = match tags
            .get("journal")
            .or(tags.get("journaltitle"))
            .map(|j| Place::Journal(Name::from(j))) 
            {
                Some(j) => j,
                None => conf.unwrap(),
            };
        let series = Series::from(tags.get("series").or(tags.get("number")));

        let date = tags.get("date");
        let year =
        if let Some(s) = date {
            s.split('-').next().unwrap()
        } else {
            tags.get("year").unwrap()
        };

        Paper {
            auth: Authors::from_string(tags.get("author").unwrap()),
            pages: tags
                .get("pages")
                .map(|s| Pages::from_string(s))
                .unwrap_or(Pages::default()),
            vol: Volume::from(tags.get("volume")),
            doi: Doi::from(tags.get("doi")),
            year: Year::from(year),
            title: Title::from(tags.get("title").unwrap()),
            place,
            url: Url::from(tags.get("url").unwrap()),
            abs: Abstract::from(tags.get("abstract").unwrap_or(&"".to_string())),
            series,
            publi: Publisher::from(tags.get("publisher")),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the bibtex file
    #[arg(short, long)]
    file_path: Option<String>,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    let path = match args.file_path {
        Some(p) => p,
        None => {
            eprintln!("No file provided");
            return Err(());
        }
    };
    let input = std::fs::read_to_string(path).unwrap();

    let bibtex = Bibtex::parse(&input).unwrap();
    let entries = bibtex.bibliographies();

    entries
        .iter()
        .map(Paper::from)
        .for_each(|p| println!("{}", p));

    return Ok(());
}
