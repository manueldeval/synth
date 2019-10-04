
window.dsp_nodes = {};

function normalize_io_name(name){
  return  name.toLowerCase().replace(/_/g, ' ')
}

function declareNode(type,node){
  console.log(node)

  var lGraphNodeType = function()
  {
    node.io_spec.inputs.forEach(input => {
		  this.addInput(normalize_io_name(input.name),"number");
    });
    node.io_spec.outputs.forEach(input => {
		  this.addOutput(normalize_io_name(input.name),"number");
    });
		this.shape = "card";
  };
  
  lGraphNodeType.title = type;
  lGraphNodeType.desc = type;
	lGraphNodeType.prototype.onExecute = function(){}
	lGraphNodeType.prototype.onRemoved = function(...e){ console.log("onRemoved: ",e)}
	lGraphNodeType.prototype.onAdded = function(...e){ console.log("onAdded: ",e)}
	lGraphNodeType.prototype.onConnectInput = function(... e){ console.log("onConnectInput: ",e)}
	lGraphNodeType.prototype.onConnectionsChange = function(... e){ console.log("onConnectionsChange: ",e)}
	LiteGraph.registerNodeType(node.classifier, lGraphNodeType);
}

function fetch_nodes_information(){
  axios.get('/dspnodes').then(function (response) {
    var datas = response.data;
    Object.keys(datas).forEach(function(val){
      declareNode(val,datas[val]);
     });
  })
}