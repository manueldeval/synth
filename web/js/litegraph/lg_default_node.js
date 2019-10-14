
define(function (require) {

  let helper = require("litegraph/lg_node_helper");

  /*
  =================================================
  DEFAULT NODE FACTORY
  =================================================
  */
  return function make_default_node(type,node,props,commandSender,dirtyFn){
    var lGraphNodeType = function()
    {
      // Generate i/o for LGraph
      node.io_spec.inputs.forEach(input => {
        this.addInput(helper.normalize_io_name(input.name),"number");
      });
      node.io_spec.outputs.forEach(input => {
        this.addOutput(helper.normalize_io_name(input.name),"number");
      });
      this.shape = "card";
      // Save extra informations
      this.synth_infos = {
        type: type,
        node_infos: node,
        id: helper.generate_id(type)
      };
      if (props) {
        this.properties = props
      }
      // Put hooks on title and pos
      Object.defineProperty( this, "title", {
        set: function(v) { this._title = v; dirtyFn() },
        get: function() { return this._title; },
        enumerable: true
      });
    };
    lGraphNodeType.color = "#2a363b";
    lGraphNodeType.bgcolor = "#3f5159";
    lGraphNodeType.groupcolor = "#3f789e";
    
    lGraphNodeType.title = type;
    lGraphNodeType.desc = type;
    lGraphNodeType.prototype.onExecute = function(){}
    lGraphNodeType.prototype.onAdded = helper.onNodeAdded;
    lGraphNodeType.prototype.onRemoved = helper.onNodeRemoved;
    lGraphNodeType.prototype.onConnectionsChange = helper.onConnectionsChange;
    lGraphNodeType.prototype.onPropertyChanged = helper.onPropertyChanged;
    lGraphNodeType.prototype.sendPropertyConfig = helper.sendPropertyConfig;
    lGraphNodeType.prototype.sendCommand = commandSender;
    
    return lGraphNodeType;

  }   

});