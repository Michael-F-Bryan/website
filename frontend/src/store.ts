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
    setTimes(state, entries: Entry[]) {
      const times: EntryMap = {};
      entries.forEach((entry: Entry) => times[entry.id] = entry);
      state.times = times;
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
      ctx.commit('setTimes', entries);
    },
  },
});
