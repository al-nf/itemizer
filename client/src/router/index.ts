import { createRouter, createWebHistory } from 'vue-router'
import ChampSelect from '../components/ChampSelect.vue'
import CalculatorView from '../views/CalculatorView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: ChampSelect,
    },
    {
      path: '/calc',
      name: 'calculator',
      component: CalculatorView,
    },
  ],
})

export default router
