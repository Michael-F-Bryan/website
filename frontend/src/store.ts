import Vue from 'vue';
import Vuex from 'vuex';
import User, { UserLevel } from '@/client/User';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    currentUser: User.Anonymous(),
  },
  mutations: {
    setUser(state, user) {
      state.currentUser = user;
    },
  },
  actions: {
    login(ctx, { username, password }) {
      const lvl = username.toLowerCase() === 'admin' ? UserLevel.Admin : UserLevel.Normal;
      const user = new User(username, window.btoa(password), lvl);

      return ctx.commit('setUser', user);
    },
    logout(ctx) {
      return ctx.commit('setUser', User.Anonymous());
    },
  },
});
