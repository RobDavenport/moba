// source: ClientMessage.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

var jspb = require('google-protobuf');
var goog = jspb;
var global = Function('return this')();

goog.exportSymbol('proto.ClientMessage', null, global);
goog.exportSymbol('proto.ClientMessage.ClientMessageType', null, global);
goog.exportSymbol('proto.ClientMessage.MoveCommand', null, global);
goog.exportSymbol('proto.ClientMessage.MsgdataCase', null, global);
goog.exportSymbol('proto.ClientMessage.VerifyRtc', null, global);
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.ClientMessage = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, proto.ClientMessage.oneofGroups_);
};
goog.inherits(proto.ClientMessage, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.ClientMessage.displayName = 'proto.ClientMessage';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.ClientMessage.MoveCommand = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.ClientMessage.MoveCommand, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.ClientMessage.MoveCommand.displayName = 'proto.ClientMessage.MoveCommand';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.ClientMessage.VerifyRtc = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.ClientMessage.VerifyRtc, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.ClientMessage.VerifyRtc.displayName = 'proto.ClientMessage.VerifyRtc';
}

/**
 * Oneof group definitions for this message. Each group defines the field
 * numbers belonging to that group. When of these fields' value is set, all
 * other fields in the group are cleared. During deserialization, if multiple
 * fields are encountered for a group, only the last value seen will be kept.
 * @private {!Array<!Array<number>>}
 * @const
 */
proto.ClientMessage.oneofGroups_ = [[2,100]];

/**
 * @enum {number}
 */
proto.ClientMessage.MsgdataCase = {
  MSGDATA_NOT_SET: 0,
  MOVECOMMAND: 2,
  VERYFIYRTC: 100
};

/**
 * @return {proto.ClientMessage.MsgdataCase}
 */
proto.ClientMessage.prototype.getMsgdataCase = function() {
  return /** @type {proto.ClientMessage.MsgdataCase} */(jspb.Message.computeOneofCase(this, proto.ClientMessage.oneofGroups_[0]));
};



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.ClientMessage.prototype.toObject = function(opt_includeInstance) {
  return proto.ClientMessage.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.ClientMessage} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ClientMessage.toObject = function(includeInstance, msg) {
  var f, obj = {
    msgtype: jspb.Message.getFieldWithDefault(msg, 1, 0),
    movecommand: (f = msg.getMovecommand()) && proto.ClientMessage.MoveCommand.toObject(includeInstance, f),
    veryfiyrtc: (f = msg.getVeryfiyrtc()) && proto.ClientMessage.VerifyRtc.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.ClientMessage}
 */
proto.ClientMessage.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.ClientMessage;
  return proto.ClientMessage.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.ClientMessage} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.ClientMessage}
 */
proto.ClientMessage.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.ClientMessage.ClientMessageType} */ (reader.readEnum());
      msg.setMsgtype(value);
      break;
    case 2:
      var value = new proto.ClientMessage.MoveCommand;
      reader.readMessage(value,proto.ClientMessage.MoveCommand.deserializeBinaryFromReader);
      msg.setMovecommand(value);
      break;
    case 100:
      var value = new proto.ClientMessage.VerifyRtc;
      reader.readMessage(value,proto.ClientMessage.VerifyRtc.deserializeBinaryFromReader);
      msg.setVeryfiyrtc(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.ClientMessage.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.ClientMessage.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.ClientMessage} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ClientMessage.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getMsgtype();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getMovecommand();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.ClientMessage.MoveCommand.serializeBinaryToWriter
    );
  }
  f = message.getVeryfiyrtc();
  if (f != null) {
    writer.writeMessage(
      100,
      f,
      proto.ClientMessage.VerifyRtc.serializeBinaryToWriter
    );
  }
};


/**
 * @enum {number}
 */
proto.ClientMessage.ClientMessageType = {
  NONE: 0,
  MOVE: 1,
  VERIFYRTC: 100
};




if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.ClientMessage.MoveCommand.prototype.toObject = function(opt_includeInstance) {
  return proto.ClientMessage.MoveCommand.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.ClientMessage.MoveCommand} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ClientMessage.MoveCommand.toObject = function(includeInstance, msg) {
  var f, obj = {
    x: jspb.Message.getFloatingPointFieldWithDefault(msg, 1, 0.0),
    y: jspb.Message.getFloatingPointFieldWithDefault(msg, 2, 0.0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.ClientMessage.MoveCommand}
 */
proto.ClientMessage.MoveCommand.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.ClientMessage.MoveCommand;
  return proto.ClientMessage.MoveCommand.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.ClientMessage.MoveCommand} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.ClientMessage.MoveCommand}
 */
proto.ClientMessage.MoveCommand.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readFloat());
      msg.setX(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readFloat());
      msg.setY(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.ClientMessage.MoveCommand.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.ClientMessage.MoveCommand.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.ClientMessage.MoveCommand} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ClientMessage.MoveCommand.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getX();
  if (f !== 0.0) {
    writer.writeFloat(
      1,
      f
    );
  }
  f = message.getY();
  if (f !== 0.0) {
    writer.writeFloat(
      2,
      f
    );
  }
};


/**
 * optional float x = 1;
 * @return {number}
 */
proto.ClientMessage.MoveCommand.prototype.getX = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 1, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.ClientMessage.MoveCommand} returns this
 */
proto.ClientMessage.MoveCommand.prototype.setX = function(value) {
  return jspb.Message.setProto3FloatField(this, 1, value);
};


/**
 * optional float y = 2;
 * @return {number}
 */
proto.ClientMessage.MoveCommand.prototype.getY = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 2, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.ClientMessage.MoveCommand} returns this
 */
proto.ClientMessage.MoveCommand.prototype.setY = function(value) {
  return jspb.Message.setProto3FloatField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.ClientMessage.VerifyRtc.prototype.toObject = function(opt_includeInstance) {
  return proto.ClientMessage.VerifyRtc.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.ClientMessage.VerifyRtc} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ClientMessage.VerifyRtc.toObject = function(includeInstance, msg) {
  var f, obj = {
    uuid: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.ClientMessage.VerifyRtc}
 */
proto.ClientMessage.VerifyRtc.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.ClientMessage.VerifyRtc;
  return proto.ClientMessage.VerifyRtc.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.ClientMessage.VerifyRtc} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.ClientMessage.VerifyRtc}
 */
proto.ClientMessage.VerifyRtc.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setUuid(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.ClientMessage.VerifyRtc.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.ClientMessage.VerifyRtc.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.ClientMessage.VerifyRtc} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ClientMessage.VerifyRtc.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getUuid();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string uuid = 1;
 * @return {string}
 */
proto.ClientMessage.VerifyRtc.prototype.getUuid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.ClientMessage.VerifyRtc} returns this
 */
proto.ClientMessage.VerifyRtc.prototype.setUuid = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional ClientMessageType msgType = 1;
 * @return {!proto.ClientMessage.ClientMessageType}
 */
proto.ClientMessage.prototype.getMsgtype = function() {
  return /** @type {!proto.ClientMessage.ClientMessageType} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.ClientMessage.ClientMessageType} value
 * @return {!proto.ClientMessage} returns this
 */
proto.ClientMessage.prototype.setMsgtype = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional MoveCommand moveCommand = 2;
 * @return {?proto.ClientMessage.MoveCommand}
 */
proto.ClientMessage.prototype.getMovecommand = function() {
  return /** @type{?proto.ClientMessage.MoveCommand} */ (
    jspb.Message.getWrapperField(this, proto.ClientMessage.MoveCommand, 2));
};


/**
 * @param {?proto.ClientMessage.MoveCommand|undefined} value
 * @return {!proto.ClientMessage} returns this
*/
proto.ClientMessage.prototype.setMovecommand = function(value) {
  return jspb.Message.setOneofWrapperField(this, 2, proto.ClientMessage.oneofGroups_[0], value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.ClientMessage} returns this
 */
proto.ClientMessage.prototype.clearMovecommand = function() {
  return this.setMovecommand(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.ClientMessage.prototype.hasMovecommand = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional VerifyRtc veryfiyRtc = 100;
 * @return {?proto.ClientMessage.VerifyRtc}
 */
proto.ClientMessage.prototype.getVeryfiyrtc = function() {
  return /** @type{?proto.ClientMessage.VerifyRtc} */ (
    jspb.Message.getWrapperField(this, proto.ClientMessage.VerifyRtc, 100));
};


/**
 * @param {?proto.ClientMessage.VerifyRtc|undefined} value
 * @return {!proto.ClientMessage} returns this
*/
proto.ClientMessage.prototype.setVeryfiyrtc = function(value) {
  return jspb.Message.setOneofWrapperField(this, 100, proto.ClientMessage.oneofGroups_[0], value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.ClientMessage} returns this
 */
proto.ClientMessage.prototype.clearVeryfiyrtc = function() {
  return this.setVeryfiyrtc(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.ClientMessage.prototype.hasVeryfiyrtc = function() {
  return jspb.Message.getField(this, 100) != null;
};


goog.object.extend(exports, proto);
