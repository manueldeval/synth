define(function (require) {

  let make_default_node = require('litegraph/lg_default_node')
  let make_knob = require('litegraph/lg_knob')

  function generate_properties(type){
    var props = { 
      Keyboard :  { osc_channel: "/keyboard" },
      Knob:       { osc_channel: "/knob", value: 0.0 },
    }
    return props[type] || {}
  }
  
  return function registerNodeType(node,commandSender,dirtyFn){
    var lGraphNodeType;
    switch(node.type){
      case "Knob":
        lGraphNodeType = make_knob(node.type, node, generate_properties(node.type),commandSender,dirtyFn);
        break
      default: 
        lGraphNodeType = make_default_node(node.type, node, generate_properties(node.type),commandSender,dirtyFn);
    }
    LiteGraph.registerNodeType(node.classifier, lGraphNodeType);
  }
  
})

