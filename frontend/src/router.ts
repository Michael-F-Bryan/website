import Vue from 'vue';
import Router from 'vue-router';
import Home from '@/views/Home.vue';
import ViewTimes from '@/views/ViewTimes.vue';
import ViewTime from '@/views/ViewTime.vue';
import AdminDashboard from '@/views/AdminDashboard.vue';
import Portfolio from '@/views/Portfolio.vue';
import Login from '@/views/Login.vue';
import store from '@/store';
import { UserLevel, isUserLevel } from './client/User';

Vue.use(Router);

const router = new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/portfolio',
      name: 'portfolio',
      component: Portfolio,
    },
    {
      path: '/times/:id',
      name: 'view-time',
      component: ViewTime,
      meta: { requiredLevel: UserLevel.Normal },
    },
    {
      path: '/times',
      name: 'view-times',
      component: ViewTimes,
      meta: { requiredLevel: UserLevel.Normal },
    },
    {
      path: '/admin',
      name: 'admin-dashboard',
      component: AdminDashboard,
      meta: { requiredLevel: UserLevel.Admin },
    },
    {
      path: '/login',
      name: 'login',
      component: Login,
    },
    {
      path: '/',
      name: 'home',
      component: Home,
    },
  ],
});

router.beforeEach((to, from, next) => {
  const expected = requiredUserLevel(to.meta);
  const currentLevel = store.state.currentUser.level;

  if (currentLevel >= expected) {
    next();
  } else {
    next(false);
  }
});

function requiredUserLevel(meta?: any): UserLevel {
  if (meta === undefined || meta.requiredLevel === undefined || !isUserLevel(meta.requiredLevel)) {
    return UserLevel.Anonymous;
  }

  return meta.requiredLevel;
}

export default router;
