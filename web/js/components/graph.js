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
      graph = new LGraph();
      canvas = new LGraphCanvas(this.$refs.graphCanvas, graph, {autoresize: true});
      canvas.getMenuOptions = function(){ return [ { content:"Audio node", has_submenu: true, callback: LGraphCanvas.onMenuAdd } ] };
      
      console.log("mounted",this.$refs.graphCanvas);
    },
    watch: {
      nodeTypes: function(nodeTypes){
        nodeTypes.forEach(nt => registerNodeType(nt));
      }
    },
    template: `<div style="height:200px;"><canvas ref="graphCanvas"></canvas></div>`,  
  });
});