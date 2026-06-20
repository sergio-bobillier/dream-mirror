import { Element } from './element'
import { Role } from './role'

export interface Character {
    id: number,
    name: string,
    portrait: string,
    element: Element,
    roles: Role[]
}