export const ROUTE_CHARACTER             = 'character';
export const ROUTE_CHARACTERS            = 'characters';
export const ROUTE_CHARACTERS_BLUEPRINTS = 'characters_blueprints';

export default [
  {
    path: '/characters/:characterId',
    name: ROUTE_CHARACTER,
    component: () => import(
      /* webpackChunkName: "character" */
      '@/characters/Character.vue'
    )
  },
  {
    path: '/characters',
    name: ROUTE_CHARACTERS,
    component: () => import(
      /* webpackChunkName: "characters" */
      '@/characters/Characters.vue'
    )
  },
  {
    path: '/characters/blueprints',
    name: ROUTE_CHARACTERS_BLUEPRINTS,
    component: () => import(
      /* webpackChunkName: "characters_blueprints" */
      '@/characters/Blueprints.vue'
    )
  },
];
