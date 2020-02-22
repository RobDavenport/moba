// source: ServerMessage.proto
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

goog.exportSymbol('proto.ServerMessage', null, global);
goog.exportSymbol('proto.ServerMessage.MsgdataCase', null, global);
goog.exportSymbol('proto.ServerMessage.ServerMessageType', null, global);
goog.exportSymbol('proto.ServerMessage.UpdateTick', null, global);
goog.exportSymbol('proto.ServerMessage.VerifiedUuid', null, global);
goog.exportSymbol('proto.ServerMessage.VerifyUuid', null, global);
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
proto.ServerMessage = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, proto.ServerMessage.oneofGroups_);
};
goog.inherits(proto.ServerMessage, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.ServerMessage.displayName = 'proto.ServerMessage';
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
proto.ServerMessage.UpdateTick = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.ServerMessage.UpdateTick, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.ServerMessage.UpdateTick.displayName = 'proto.ServerMessage.UpdateTick';
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
proto.ServerMessage.VerifyUuid = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.ServerMessage.VerifyUuid, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.ServerMessage.VerifyUuid.displayName = 'proto.ServerMessage.VerifyUuid';
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
proto.ServerMessage.VerifiedUuid = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.ServerMessage.VerifiedUuid, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.ServerMessage.VerifiedUuid.displayName = 'proto.ServerMessage.VerifiedUuid';
}

/**
 * Oneof group definitions for this message. Each group defines the field
 * numbers belonging to that group. When of these fields' value is set, all
 * other fields in the group are cleared. During deserialization, if multiple
 * fields are encountered for a group, only the last value seen will be kept.
 * @private {!Array<!Array<number>>}
 * @const
 */
proto.ServerMessage.oneofGroups_ = [[2,100,101]];

/**
 * @enum {number}
 */
proto.ServerMessage.MsgdataCase = {
  MSGDATA_NOT_SET: 0,
  UPDATETICK: 2,
  VERIFYUUID: 100,
  VERIFIEDUUID: 101
};

/**
 * @return {proto.ServerMessage.MsgdataCase}
 */
proto.ServerMessage.prototype.getMsgdataCase = function() {
  return /** @type {proto.ServerMessage.MsgdataCase} */(jspb.Message.computeOneofCase(this, proto.ServerMessage.oneofGroups_[0]));
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
proto.ServerMessage.prototype.toObject = function(opt_includeInstance) {
  return proto.ServerMessage.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.ServerMessage} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ServerMessage.toObject = function(includeInstance, msg) {
  var f, obj = {
    msgtype: jspb.Message.getFieldWithDefault(msg, 1, 0),
    updatetick: (f = msg.getUpdatetick()) && proto.ServerMessage.UpdateTick.toObject(includeInstance, f),
    verifyuuid: (f = msg.getVerifyuuid()) && proto.ServerMessage.VerifyUuid.toObject(includeInstance, f),
    verifieduuid: (f = msg.getVerifieduuid()) && proto.ServerMessage.VerifiedUuid.toObject(includeInstance, f)
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
 * @return {!proto.ServerMessage}
 */
proto.ServerMessage.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.ServerMessage;
  return proto.ServerMessage.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.ServerMessage} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.ServerMessage}
 */
proto.ServerMessage.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.ServerMessage.ServerMessageType} */ (reader.readEnum());
      msg.setMsgtype(value);
      break;
    case 2:
      var value = new proto.ServerMessage.UpdateTick;
      reader.readMessage(value,proto.ServerMessage.UpdateTick.deserializeBinaryFromReader);
      msg.setUpdatetick(value);
      break;
    case 100:
      var value = new proto.ServerMessage.VerifyUuid;
      reader.readMessage(value,proto.ServerMessage.VerifyUuid.deserializeBinaryFromReader);
      msg.setVerifyuuid(value);
      break;
    case 101:
      var value = new proto.ServerMessage.VerifiedUuid;
      reader.readMessage(value,proto.ServerMessage.VerifiedUuid.deserializeBinaryFromReader);
      msg.setVerifieduuid(value);
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
proto.ServerMessage.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.ServerMessage.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.ServerMessage} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ServerMessage.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getMsgtype();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getUpdatetick();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.ServerMessage.UpdateTick.serializeBinaryToWriter
    );
  }
  f = message.getVerifyuuid();
  if (f != null) {
    writer.writeMessage(
      100,
      f,
      proto.ServerMessage.VerifyUuid.serializeBinaryToWriter
    );
  }
  f = message.getVerifieduuid();
  if (f != null) {
    writer.writeMessage(
      101,
      f,
      proto.ServerMessage.VerifiedUuid.serializeBinaryToWriter
    );
  }
};


/**
 * @enum {number}
 */
proto.ServerMessage.ServerMessageType = {
  NONE: 0,
  UPDATETICK: 1,
  VERIFYUUID: 2,
  VERIFIEDUUID: 3
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
proto.ServerMessage.UpdateTick.prototype.toObject = function(opt_includeInstance) {
  return proto.ServerMessage.UpdateTick.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.ServerMessage.UpdateTick} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ServerMessage.UpdateTick.toObject = function(includeInstance, msg) {
  var f, obj = {
    frame: jspb.Message.getFloatingPointFieldWithDefault(msg, 1, 0.0),
    entity: jspb.Message.getFieldWithDefault(msg, 2, 0),
    x: jspb.Message.getFloatingPointFieldWithDefault(msg, 3, 0.0),
    y: jspb.Message.getFloatingPointFieldWithDefault(msg, 4, 0.0)
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
 * @return {!proto.ServerMessage.UpdateTick}
 */
proto.ServerMessage.UpdateTick.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.ServerMessage.UpdateTick;
  return proto.ServerMessage.UpdateTick.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.ServerMessage.UpdateTick} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.ServerMessage.UpdateTick}
 */
proto.ServerMessage.UpdateTick.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readFloat());
      msg.setFrame(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setEntity(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readFloat());
      msg.setX(value);
      break;
    case 4:
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
proto.ServerMessage.UpdateTick.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.ServerMessage.UpdateTick.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.ServerMessage.UpdateTick} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ServerMessage.UpdateTick.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getFrame();
  if (f !== 0.0) {
    writer.writeFloat(
      1,
      f
    );
  }
  f = message.getEntity();
  if (f !== 0) {
    writer.writeUint32(
      2,
      f
    );
  }
  f = message.getX();
  if (f !== 0.0) {
    writer.writeFloat(
      3,
      f
    );
  }
  f = message.getY();
  if (f !== 0.0) {
    writer.writeFloat(
      4,
      f
    );
  }
};


/**
 * optional float frame = 1;
 * @return {number}
 */
proto.ServerMessage.UpdateTick.prototype.getFrame = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 1, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.ServerMessage.UpdateTick} returns this
 */
proto.ServerMessage.UpdateTick.prototype.setFrame = function(value) {
  return jspb.Message.setProto3FloatField(this, 1, value);
};


/**
 * optional uint32 entity = 2;
 * @return {number}
 */
proto.ServerMessage.UpdateTick.prototype.getEntity = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.ServerMessage.UpdateTick} returns this
 */
proto.ServerMessage.UpdateTick.prototype.setEntity = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional float x = 3;
 * @return {number}
 */
proto.ServerMessage.UpdateTick.prototype.getX = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 3, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.ServerMessage.UpdateTick} returns this
 */
proto.ServerMessage.UpdateTick.prototype.setX = function(value) {
  return jspb.Message.setProto3FloatField(this, 3, value);
};


/**
 * optional float y = 4;
 * @return {number}
 */
proto.ServerMessage.UpdateTick.prototype.getY = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 4, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.ServerMessage.UpdateTick} returns this
 */
proto.ServerMessage.UpdateTick.prototype.setY = function(value) {
  return jspb.Message.setProto3FloatField(this, 4, value);
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
proto.ServerMessage.VerifyUuid.prototype.toObject = function(opt_includeInstance) {
  return proto.ServerMessage.VerifyUuid.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.ServerMessage.VerifyUuid} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ServerMessage.VerifyUuid.toObject = function(includeInstance, msg) {
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
 * @return {!proto.ServerMessage.VerifyUuid}
 */
proto.ServerMessage.VerifyUuid.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.ServerMessage.VerifyUuid;
  return proto.ServerMessage.VerifyUuid.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.ServerMessage.VerifyUuid} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.ServerMessage.VerifyUuid}
 */
proto.ServerMessage.VerifyUuid.deserializeBinaryFromReader = function(msg, reader) {
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
proto.ServerMessage.VerifyUuid.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.ServerMessage.VerifyUuid.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.ServerMessage.VerifyUuid} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ServerMessage.VerifyUuid.serializeBinaryToWriter = function(message, writer) {
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
proto.ServerMessage.VerifyUuid.prototype.getUuid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.ServerMessage.VerifyUuid} returns this
 */
proto.ServerMessage.VerifyUuid.prototype.setUuid = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
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
proto.ServerMessage.VerifiedUuid.prototype.toObject = function(opt_includeInstance) {
  return proto.ServerMessage.VerifiedUuid.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.ServerMessage.VerifiedUuid} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ServerMessage.VerifiedUuid.toObject = function(includeInstance, msg) {
  var f, obj = {

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
 * @return {!proto.ServerMessage.VerifiedUuid}
 */
proto.ServerMessage.VerifiedUuid.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.ServerMessage.VerifiedUuid;
  return proto.ServerMessage.VerifiedUuid.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.ServerMessage.VerifiedUuid} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.ServerMessage.VerifiedUuid}
 */
proto.ServerMessage.VerifiedUuid.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
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
proto.ServerMessage.VerifiedUuid.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.ServerMessage.VerifiedUuid.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.ServerMessage.VerifiedUuid} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ServerMessage.VerifiedUuid.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};


/**
 * optional ServerMessageType msgType = 1;
 * @return {!proto.ServerMessage.ServerMessageType}
 */
proto.ServerMessage.prototype.getMsgtype = function() {
  return /** @type {!proto.ServerMessage.ServerMessageType} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.ServerMessage.ServerMessageType} value
 * @return {!proto.ServerMessage} returns this
 */
proto.ServerMessage.prototype.setMsgtype = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional UpdateTick updateTick = 2;
 * @return {?proto.ServerMessage.UpdateTick}
 */
proto.ServerMessage.prototype.getUpdatetick = function() {
  return /** @type{?proto.ServerMessage.UpdateTick} */ (
    jspb.Message.getWrapperField(this, proto.ServerMessage.UpdateTick, 2));
};


/**
 * @param {?proto.ServerMessage.UpdateTick|undefined} value
 * @return {!proto.ServerMessage} returns this
*/
proto.ServerMessage.prototype.setUpdatetick = function(value) {
  return jspb.Message.setOneofWrapperField(this, 2, proto.ServerMessage.oneofGroups_[0], value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.ServerMessage} returns this
 */
proto.ServerMessage.prototype.clearUpdatetick = function() {
  return this.setUpdatetick(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.ServerMessage.prototype.hasUpdatetick = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional VerifyUuid verifyUuid = 100;
 * @return {?proto.ServerMessage.VerifyUuid}
 */
proto.ServerMessage.prototype.getVerifyuuid = function() {
  return /** @type{?proto.ServerMessage.VerifyUuid} */ (
    jspb.Message.getWrapperField(this, proto.ServerMessage.VerifyUuid, 100));
};


/**
 * @param {?proto.ServerMessage.VerifyUuid|undefined} value
 * @return {!proto.ServerMessage} returns this
*/
proto.ServerMessage.prototype.setVerifyuuid = function(value) {
  return jspb.Message.setOneofWrapperField(this, 100, proto.ServerMessage.oneofGroups_[0], value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.ServerMessage} returns this
 */
proto.ServerMessage.prototype.clearVerifyuuid = function() {
  return this.setVerifyuuid(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.ServerMessage.prototype.hasVerifyuuid = function() {
  return jspb.Message.getField(this, 100) != null;
};


/**
 * optional VerifiedUuid verifiedUuid = 101;
 * @return {?proto.ServerMessage.VerifiedUuid}
 */
proto.ServerMessage.prototype.getVerifieduuid = function() {
  return /** @type{?proto.ServerMessage.VerifiedUuid} */ (
    jspb.Message.getWrapperField(this, proto.ServerMessage.VerifiedUuid, 101));
};


/**
 * @param {?proto.ServerMessage.VerifiedUuid|undefined} value
 * @return {!proto.ServerMessage} returns this
*/
proto.ServerMessage.prototype.setVerifieduuid = function(value) {
  return jspb.Message.setOneofWrapperField(this, 101, proto.ServerMessage.oneofGroups_[0], value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.ServerMessage} returns this
 */
proto.ServerMessage.prototype.clearVerifieduuid = function() {
  return this.setVerifieduuid(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.ServerMessage.prototype.hasVerifieduuid = function() {
  return jspb.Message.getField(this, 101) != null;
};


goog.object.extend(exports, proto);