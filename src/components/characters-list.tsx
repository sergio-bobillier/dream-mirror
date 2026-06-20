import { use } from "react";

import { Character } from "../types/character"
import CharacterCard from "./character-card";

export default function CharactersList({characters}: {characters: Promise<Character[]>}) {
    const allCharacters = use(characters)

    return (
        <div className="row row-cards">
            {allCharacters.map((character) => (
                <CharacterCard {...character} />
            ))}
        </div>
    )
}