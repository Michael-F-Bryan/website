export default class Api {
    constructor(config) {
        this.api_root = "/api";
        this.times = [];

        if (config) {
            Object.assign(this, config);
        }
    }

    async ping() {
        if (!this.token) {
            return false;
        }

        const response = await fetch(this.api_root + "/ping", {token: this.token});
        const jason = await response.json();

        return !jason.error;
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

    async summary() {
        const response = await fetch(this.api_root + "/times/summary", { token: this.token });
        const summary = await response.json();

        if (summary.error) {
            throw new Error(summary.error);
        } else {
            const whitelist = ["times", "total-hours", "average-day"];

            return Object.keys(summary)
                .filter(key => whitelist.includes(key))
                .reduce((obj, key) => {
                    obj[key] = summary[key];
                    return obj;
                }, {});
        }
    }
}
