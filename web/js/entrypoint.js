require.config({
  paths: {
      vue: '../node_modules/vue/dist/vue',
      axios: '../node_modules/axios/dist/axios'
  }
});

define(function (require) {

  let Vue = require('vue');
  let Store = require('store/appstore')

  require('components/app');

  // Init at startup
  Store.fetchNodeType();

  // Launch
  new Vue({
      el: '#main',
      template: "<App></App>"
  });
});