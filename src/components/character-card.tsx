import Image from "next/image"

import { Character } from '../types/character'

export default function CharacterCard(character: Character) {
    return (
        <div key={character.id} className="col-md-6 col-lg-3">
            <div className="card">
                <div className="card-body p-4 text-center">
                    <span className="avatar avatar-2xl mb-3" style={{ backgroundImage: `url(/${character.portrait})` }}></span>
                    <h3 className="m-0 mb-1"><a href="#">{ character.name }</a></h3>
                    <div className="text-secondary">
                        <Image alt={character.element.name} src={`/${character.element.icon}`} height="16" width="16" className="me-1" />
                        {character.element.name}
                    </div>
                    <div className="mt-3">
                        {character.roles.map((role, index) => {
                            let margin = index > 0 ? 'ms-1' : null;
                            return <span key={role.id} className={`${margin} badge bg-${role.color}-lt`}>{role.name}</span>
                        })}
                    </div>
                </div>
            </div>
        </div>
    )
}