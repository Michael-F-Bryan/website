export default class User {
    public static Anonymous(): User {
        return new User('', '', UserLevel.Anonymous);
    }

    public readonly username: string;
    public readonly id: string;
    public readonly level: UserLevel;

    constructor(username: string, id: string, level: UserLevel) {
        this.username = username;
        this.id = id;
        this.level = level;
    }

    get isAdmin(): boolean {
        return this.level === UserLevel.Admin;
    }

    get isLoggedIn(): boolean {
        return this.level === UserLevel.Normal || this.level === UserLevel.Admin;
    }
}

export enum UserLevel {
    Anonymous = 0,
    Normal = 1,
    Admin = 2,
}

export function isUserLevel(thing?: any): thing is UserLevel {
    return thing === UserLevel.Anonymous || thing === UserLevel.Normal || thing === UserLevel.Admin;
}
