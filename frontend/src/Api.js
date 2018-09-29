export default class Api {
    constructor(api_root) {
        this.api_root = api_root || "/api";
    }

    async login(username, password) {
        const response = await fetch(this.api_root + "/login", {username, password});
        const { token, error } = await response.json();

        if (error) {
            throw new Error(error);
        } else {
            this.token = token;
        }
    }
}
