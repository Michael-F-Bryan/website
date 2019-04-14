import Vue from 'vue';
import Vuex from 'vuex';
import User, { UserLevel } from '@/client/User';
import Entry from '@/client/Entry';

Vue.use(Vuex);

interface EntryMap {
  [id: string]: Entry;
}

export default new Vuex.Store({
  state: {
    currentUser: User.Anonymous(),
    times: {} as EntryMap,
  },
  mutations: {
    setUser(state, user) {
      state.currentUser = user;
    },
    updateTimes(state, entries: Entry[]) {
      entries.forEach((entry: Entry) => Vue.set(state.times, entry.id, entry));
    },
    updateTime(state, entry: Entry) {
      Vue.set(state.times, entry.id, entry);
    },
  },
  actions: {
    login(ctx, { username, password }) {
      const lvl = username.toLowerCase() === 'admin' ? UserLevel.Admin : UserLevel.Normal;
      const user = new User(username, window.btoa(password), lvl);
      ctx.commit('setUser', user);
    },
    logout(ctx) {
      ctx.commit('setUser', User.Anonymous());
    },
    fetchTimes(ctx) {
      const entries = [new Entry('first', new Date('2019-01-01 08:00'), new Date('2019-01-01 17:00'))];
      ctx.commit('updateTimes', entries);
    },
    updateTime(ctx, { start, end, breaks, description, id }): Entry {
      if (!id || id.length === 0) {
        // no ID was provided, generate a "random" one
        id = btoa(start.toString() + end.toString() + description);
      }

      const entry = new Entry(id, start, end, description, breaks);
      ctx.commit('updateTime', entry);
      return entry;
    },
  },
});
