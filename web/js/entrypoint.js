require.config({
  paths: {
      vue: '../node_modules/vue/dist/vue',
      axios: '../node_modules/axios/dist/axios',
      'bootstrap-vue': '../node_modules/bootstrap-vue/dist/bootstrap-vue'
  }
});

define(function (require) {

  // Vue stuff
  let Vue = require('vue');
  let BootstrapVue =  require('bootstrap-vue');
  Vue.use(BootstrapVue);

  // Load app state
  let Store = require('store/appstore')
  // Init at startup
  Store.fetchNodeType();


  require('components/app');
  // Launch
  new Vue({
      el: '#main',
      template: "<App></App>"
  });
});