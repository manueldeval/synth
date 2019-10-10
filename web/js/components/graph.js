define(function (require) {
  let Vue = require('vue');

  let Store = require('store/appstore')
  let registerNodeType = require('litegraph/lg_nodefactory');

  Vue.component('Graph', {
    data:  function () {
      return { 
        nodeTypes: Store.state.nodeTypes
      }
    },
    mounted: function() {
      canvas = new LGraphCanvas(this.$refs.graphCanvas, Store.getGraph(), {autoresize: true});
      canvas.getMenuOptions = function(){ return [ { content:"Audio node", has_submenu: true, callback: LGraphCanvas.onMenuAdd } ] };      
    },
    watch: {
      nodeTypes: function(nodeTypes){
        nodeTypes.forEach(nt => registerNodeType(nt));
      }
    },
    template: `<div style="height:300px;"><canvas ref="graphCanvas"></canvas></div>`,  
  });
});