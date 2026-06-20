use std::io::stdout;
use thiserror::Error;

use super::DB;
use crate::console::Logger;
use crate::console::Severity;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to seed the database: {0}")]
    SchemaLoadingFailed(#[source] sqlite::Error)
}

fn insert_records(db: &DB, table: &str, records: &[(&str, &str)]) -> Result<(), Error> {
    let mut logger = Logger::new(stdout());

    for record in records {
        let statement = format!("INSERT INTO {} ({}) VALUES ({});", table, record.0, record.1);
        let result = db.connection.execute(statement);

        if let Err(error) = result {
            return Err(Error::SchemaLoadingFailed(error));
        }
    }

    logger.log_message(format!("Done. Inserted {} records.", records.len()), Severity::Debug);

    Ok(())
}

pub fn seed(db: &DB) -> Result<(), Error> {
    let mut logger = Logger::new(stdout());

    // Elements

    logger.debug("Seeding 'Elements' table...");

    let records = [
        ("id, name, icon, color", "1, 'Pyro', 'img/icons/pyro.webp', 'red'"),
        ("id, name, icon, color", "2, 'Anemo', 'img/icons/anemo.webp', 'cyan'"),
        ("id, name, icon, color", "3, 'Cryo', 'img/icons/cryo.webp', 'azure'"),
        ("id, name, icon, color", "4, 'Hydro', 'img/icons/hydro.webp', 'blue'"),
        ("id, name, icon, color", "5, 'Dendro', 'img/icons/dendro.webp', 'green'"),
        ("id, name, icon, color", "6, 'Electro', 'img/icons/electro.webp', 'purple'"),
        ("id, name, icon, color", "7, 'Geo', 'img/icons/geo.webp', 'yellow'")
    ];

    insert_records(db, "elements", &records)?;

    // Roles

    logger.debug("Seeding 'Roles' table...");

    let records = [
        ("id, name, icon, color", "1, 'Off-field', 'img/icons/off-field.webp', 'blue'"),
        ("id, name, icon, color", "2, 'Support', 'img/icons/support.webp', 'purple'"),
        ("id, name, icon, color", "3, 'Healer', 'img/icons/healer.webp', 'green'"),
        ("id, name, icon, color", "4, 'On-field', 'img/icons/on-field.webp', 'yellow'"),
        ("id, name, icon, color", "5, 'DPS', 'img/icons/dps.webp', 'red'")
    ];

    insert_records(db, "roles", &records)?;

    // Characters

    logger.debug("Seeding 'Characters' table...");

    let records = [
        ("id, name, portrait, element_id", "1, 'Nicole', 'img/characters/nicole.webp', 1"),
        ("id, name, portrait, element_id", "2, 'Jahoda', 'img/characters/jahoda.webp', 2"),
        ("id, name, portrait, element_id", "3, 'Sandrone', 'img/characters/sandrone.webp', 3"),
        ("id, name, portrait, element_id", "4, 'Citlali', 'img/characters/citlali.webp', 3"),
        ("id, name, portrait, element_id", "5, 'Yoimiya', 'img/characters/yoimiya.webp', 1"),
        ("id, name, portrait, element_id", "6, 'Sangonomiya Kokomi', 'img/characters/kokomi.webp', 4"),
        ("id, name, portrait, element_id", "7, 'Yaoyao', 'img/characters/yaoyao.webp', 5"),
        ("id, name, portrait, element_id", "8, 'Chevreuse', 'img/characters/chevreuse.webp', 1"),
        ("id, name, portrait, element_id", "9, 'Fischl', 'img/characters/fischl.webp', 6"),
        ("id, name, portrait, element_id", "10, 'Sucrose', 'img/characters/sucrose.webp', 2"),
        ("id, name, portrait, element_id", "11, 'Kuki Shinobu', 'img/characters/shinobu.webp', 6"),
        ("id, name, portrait, element_id", "12, 'Qiqi', 'img/characters/qiqi.webp', 3"),
        ("id, name, portrait, element_id", "13, 'Lynette', 'img/characters/lynette.webp', 2"),
        ("id, name, portrait, element_id", "14, 'Escoffier', 'img/characters/escoffier.webp', 3"),
        ("id, name, portrait, element_id", "15, 'Kachina', 'img/characters/kachina.webp', 7"),
        ("id, name, portrait, element_id", "16, 'Collei', 'img/characters/collei.webp', 5"),
        ("id, name, portrait, element_id", "17, 'Xiangling', 'img/characters/xiangling.webp', 1"),
        ("id, name, portrait, element_id", "18, 'Yanfei', 'img/characters/yanfei.webp', 1"),
        ("id, name, portrait, element_id", "19, 'Rosaria', 'img/characters/rosaria.webp', 3"),
        ("id, name, portrait, element_id", "20, 'Noelle', 'img/characters/noelle.webp', 7"),
        ("id, name, portrait, element_id", "21, 'Kirara', 'img/characters/kirara.webp', 5"),
        ("id, name, portrait, element_id", "22, 'Keqing', 'img/characters/keqing.webp', 6"),
        ("id, name, portrait, element_id", "23, 'Ningguang', 'img/characters/ningguang.webp', 7"),
        ("id, name, portrait, element_id", "24, 'Barbara', 'img/characters/barbara.webp', 4"),
        ("id, name, portrait, element_id", "25, 'Diona', 'img/characters/diona.webp', 3"),
        ("id, name, portrait, element_id", "26, 'Faruzan', 'img/characters/faruzan.webp', 2"),
        ("id, name, portrait, element_id", "27, 'Lumine', 'img/characters/lumine.webp', 7"),
        ("id, name, portrait, element_id", "28, 'Sayu', 'img/characters/sayu.webp', 2"),
        ("id, name, portrait, element_id", "29, 'Lumine', 'img/characters/lumine.webp', 1"),
        ("id, name, portrait, element_id", "30, 'Lisa', 'img/characters/lisa.webp', 6"),
        ("id, name, portrait, element_id", "31, 'Lumine', 'img/characters/lumine.webp', 5"),
        ("id, name, portrait, element_id", "32, 'Lumine', 'img/characters/lumine.webp', 2"),
        ("id, name, portrait, element_id", "33, 'Furina', 'img/characters/furina.webp', 4"),
        ("id, name, portrait, element_id", "34, 'Skirk', 'img/characters/skirk.webp', 3"),
        ("id, name, portrait, element_id", "35, 'Lan Yan', 'img/characters/lan-yan.webp', 2"),
        ("id, name, portrait, element_id", "36, 'Raiden Shogun', 'img/characters/raiden.webp', 6"),
        ("id, name, portrait, element_id", "37, 'Nilou', 'img/characters/nilou.webp', 4"),
    ];

    insert_records(db, "characters", &records)?;

    // Character Roles

    logger.debug("Seeding 'Character Roles' table...");

    let records = [
        ("character_id, role_id", "1, 1"),   // Nicole: Off-field
        ("character_id, role_id", "1, 2"),   //       : Support

        ("character_id, role_id", "2, 1"),   // Jahoda: Off-field
        ("character_id, role_id", "2, 2"),   //       : Support
        ("character_id, role_id", "2, 3"),   //       : Healer

        ("character_id, role_id", "3, 4"),   // Sandrone: On-field
        ("character_id, role_id", "3, 5"),   //         : DPS

        ("character_id, role_id", "4, 1"),   // Citlali: Off-field
        ("character_id, role_id", "4, 2"),   //        : Support

        ("character_id, role_id", "5, 4"),   // Yoimiya: On-field
        ("character_id, role_id", "5, 5"),   //        : DPS

        ("character_id, role_id", "6, 1"),   // Kokomi: Off-field
        ("character_id, role_id", "6, 2"),   //       : Support
        ("character_id, role_id", "6, 3"),   //       : Healer

        ("character_id, role_id", "7, 1"),   // Yaoyao: Off-field
        ("character_id, role_id", "7, 3"),   //       : Healer

        ("character_id, role_id", "8, 1"),   // Chevreuse: Off-field
        ("character_id, role_id", "8, 2"),   //          : Support
        ("character_id, role_id", "8, 3"),   //          : Healer

        ("character_id, role_id", "9, 1"),   // Fischl: Off-field
        ("character_id, role_id", "9, 5"),   //       : DPS

        ("character_id, role_id", "10, 1"),   // Sucrose: Off-field
        ("character_id, role_id", "10, 2"),   //        : Support
        ("character_id, role_id", "10, 5"),   //        : DPS

        ("character_id, role_id", "11, 1"),   // Shinobu: Off-field
        ("character_id, role_id", "11, 3"),   //        : Healer

        ("character_id, role_id", "12, 1"),   // Qiqi: Off-field
        ("character_id, role_id", "12, 3"),   //     : Healer

        ("character_id, role_id", "13, 1"),   // Lynette: Off-field
        ("character_id, role_id", "13, 5"),   //        : DPS

        ("character_id, role_id", "14, 1"),   // Escoffier: Off-field
        ("character_id, role_id", "14, 2"),   //          : Support
        ("character_id, role_id", "14, 3"),   //          : Healer
        ("character_id, role_id", "14, 5"),   //          : DPS

        ("character_id, role_id", "15, 1"),   // Kachina: Off-field
        ("character_id, role_id", "15, 5"),   //        : DPS

        ("character_id, role_id", "16, 1"),   // Collei: Off-field
        ("character_id, role_id", "16, 5"),   //       : DPS

        ("character_id, role_id", "17, 1"),   // Xiangling: Off-field
        ("character_id, role_id", "17, 5"),   //          : DPS

        ("character_id, role_id", "18, 4"),   // Yanfei: On-field
        ("character_id, role_id", "18, 5"),   //       : DPS

        ("character_id, role_id", "19, 1"),   // Rosaria: Off-field
        ("character_id, role_id", "19, 5"),   //        : DPS

        ("character_id, role_id", "20, 4"),   // Noelle: On-field
        ("character_id, role_id", "20, 5"),   //       : DPS
        ("character_id, role_id", "20, 3"),   //       : Healer

        ("character_id, role_id", "21, 1"),   // Kirara: Off-field
        ("character_id, role_id", "21, 2"),   //       : Support

        ("character_id, role_id", "22, 4"),   // Keqing: On-field
        ("character_id, role_id", "22, 5"),   //       : DPS

        ("character_id, role_id", "23, 4"),   // Ningguang: On-field
        ("character_id, role_id", "23, 5"),   //          : DPS

        ("character_id, role_id", "24, 1"),   // Barbara: Off-field
        ("character_id, role_id", "24, 3"),   //        : Healer

        ("character_id, role_id", "25, 1"),   // Diona: Off-field
        ("character_id, role_id", "25, 3"),   //      : Healer

        ("character_id, role_id", "26, 1"),   // Faruzan: Off-field
        ("character_id, role_id", "26, 2"),   //        : Support

        ("character_id, role_id", "27, 1"),   // Lumine (Geo): Off-field
        ("character_id, role_id", "27, 5"),   //             : DPS

        ("character_id, role_id", "28, 1"),   // Sayu: Off-field
        ("character_id, role_id", "28, 3"),   //     : Healer

        ("character_id, role_id", "29, 1"),   // Lumine (Pyro): Off-field
        ("character_id, role_id", "29, 5"),   //              : DPS

        ("character_id, role_id", "30, 1"),   // Lisa: Off-field
        ("character_id, role_id", "30, 5"),   //     : DPS

        ("character_id, role_id", "31, 1"),   // Lumine (Dendro): Off-field
        ("character_id, role_id", "31, 5"),   //                : DPS

        ("character_id, role_id", "32, 4"),   // Lumine (Anemo): On-field
        ("character_id, role_id", "32, 5"),   //               : DPS

        ("character_id, role_id", "33, 1"),   // Furina: Off-field
        ("character_id, role_id", "33, 2"),   //       : Support
        ("character_id, role_id", "33, 3"),   //       : Healer
        ("character_id, role_id", "33, 5"),   //       : DPS

        ("character_id, role_id", "34, 4"),   // Skirk: On-field
        ("character_id, role_id", "34, 5"),   //      : DPS

        ("character_id, role_id", "35, 1"),   // Lan Yan: Off-field
        ("character_id, role_id", "35, 2"),   //        : Support
        ("character_id, role_id", "35, 5"),   //        : DPS

        ("character_id, role_id", "36, 1"),   // Raiden Shogun: Off-field
        ("character_id, role_id", "36, 2"),   //              : Support
        ("character_id, role_id", "36, 5"),   //              : DPS

        ("character_id, role_id", "37, 4"),   // Nilou: On-field
        ("character_id, role_id", "37, 5"),   //      : DPS
        ("character_id, role_id", "37, 2"),   //      : Support
    ];

    insert_records(db, "character_roles", &records)?;

    Ok(())
}
