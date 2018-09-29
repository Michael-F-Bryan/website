import Api from "./Api";

describe("API interface", () => {
    beforeEach(() => {
        fetch.resetMocks();
    });

    it("can be instantiated", () => {
        const api = new Api();
        expect(api.token).toBeFalsy();
    });

    it("can log in successfully", async () => {
        const api = new Api();
        const expectedToken = "asd123";
        const username = "admin";
        const password = "password1";
        fetch.mockResponseOnce(JSON.stringify({ token: expectedToken }));

        await api.login(username, password);

        expect(api.token).toBe(expectedToken);
        expect(fetch.mock.calls.length).toEqual(1);
        const [url, body] = fetch.mock.calls[0];
        expect(url).toEqual("/api/login");
        expect(body).toEqual({ username, password });
    });

    it("Blows up on network errors", async () => {
        expect.assertions(3);
        const api = new Api();
        fetch.mockReject(new Error("fake error message"));

        try {
            await api.login("admin", "password1");
        } catch (e) {
            expect(e.message).toMatch("error");
        }

        expect(api.token).toBeFalsy();
        expect(fetch.mock.calls.length).toEqual(1);
    });

    it("Blows up on failed login", async () => {
        expect.assertions(3);
        const api = new Api();
        fetch.mockResponseOnce(JSON.stringify({ error: "Invalid username or password" }));

        try {
            await api.login("admin", "password1");
        } catch (e) {
            expect(e.message).toMatch("Invalid username or password");
        }

        expect(api.token).toBeFalsy();
        expect(fetch.mock.calls.length).toEqual(1);
    });

    it("Can fetch whitelisted summary", async () => {
        const api = new Api();
        fetch.mockResponseOnce(JSON.stringify({ error: "Invalid username or password" }));

    });

    it("Will blow up when fetching summary with invalid token", async () => {
        expect.assertions(1);
        const api = new Api();
        fetch.mockResponseOnce(JSON.stringify({ error: "Invalid token" }));

        try {
            const summary = await api.summary();
        } catch (e) {
            expect(e.message).toMatch("Invalid token");
        }
    });

    it("Can fetch whitelisted summary", async () => {
        const api = new Api();
        fetch.mockResponseOnce(JSON.stringify({
            "times": 5,
            "total-hours": 40.0,
            "average-day": 8.0,
            "non-existent-key": "trolololol"
        }));

        const got = await api.summary();

        const shouldBe = {
            "times": 5,
            "total-hours": 40.0,
            "average-day": 8.0
        };
        expect(got).toEqual(shouldBe);
    });
});
