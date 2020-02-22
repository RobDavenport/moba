// package: 
// file: ServerMessage.proto

import * as jspb from "google-protobuf";

export class ServerMessage extends jspb.Message {
  getMsgtype(): ServerMessage.ServerMessageTypeMap[keyof ServerMessage.ServerMessageTypeMap];
  setMsgtype(value: ServerMessage.ServerMessageTypeMap[keyof ServerMessage.ServerMessageTypeMap]): void;

  hasUpdatetick(): boolean;
  clearUpdatetick(): void;
  getUpdatetick(): ServerMessage.UpdateTick | undefined;
  setUpdatetick(value?: ServerMessage.UpdateTick): void;

  hasVerifyuuid(): boolean;
  clearVerifyuuid(): void;
  getVerifyuuid(): ServerMessage.VerifyUuid | undefined;
  setVerifyuuid(value?: ServerMessage.VerifyUuid): void;

  hasVerifieduuid(): boolean;
  clearVerifieduuid(): void;
  getVerifieduuid(): ServerMessage.VerifiedUuid | undefined;
  setVerifieduuid(value?: ServerMessage.VerifiedUuid): void;

  getMsgdataCase(): ServerMessage.MsgdataCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ServerMessage.AsObject;
  static toObject(includeInstance: boolean, msg: ServerMessage): ServerMessage.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ServerMessage, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ServerMessage;
  static deserializeBinaryFromReader(message: ServerMessage, reader: jspb.BinaryReader): ServerMessage;
}

export namespace ServerMessage {
  export type AsObject = {
    msgtype: ServerMessage.ServerMessageTypeMap[keyof ServerMessage.ServerMessageTypeMap],
    updatetick?: ServerMessage.UpdateTick.AsObject,
    verifyuuid?: ServerMessage.VerifyUuid.AsObject,
    verifieduuid?: ServerMessage.VerifiedUuid.AsObject,
  }

  export class UpdateTick extends jspb.Message {
    getFrame(): number;
    setFrame(value: number): void;

    getEntity(): number;
    setEntity(value: number): void;

    getX(): number;
    setX(value: number): void;

    getY(): number;
    setY(value: number): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): UpdateTick.AsObject;
    static toObject(includeInstance: boolean, msg: UpdateTick): UpdateTick.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: UpdateTick, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): UpdateTick;
    static deserializeBinaryFromReader(message: UpdateTick, reader: jspb.BinaryReader): UpdateTick;
  }

  export namespace UpdateTick {
    export type AsObject = {
      frame: number,
      entity: number,
      x: number,
      y: number,
    }
  }

  export class VerifyUuid extends jspb.Message {
    getUuid(): string;
    setUuid(value: string): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): VerifyUuid.AsObject;
    static toObject(includeInstance: boolean, msg: VerifyUuid): VerifyUuid.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: VerifyUuid, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): VerifyUuid;
    static deserializeBinaryFromReader(message: VerifyUuid, reader: jspb.BinaryReader): VerifyUuid;
  }

  export namespace VerifyUuid {
    export type AsObject = {
      uuid: string,
    }
  }

  export class VerifiedUuid extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): VerifiedUuid.AsObject;
    static toObject(includeInstance: boolean, msg: VerifiedUuid): VerifiedUuid.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: VerifiedUuid, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): VerifiedUuid;
    static deserializeBinaryFromReader(message: VerifiedUuid, reader: jspb.BinaryReader): VerifiedUuid;
  }

  export namespace VerifiedUuid {
    export type AsObject = {
    }
  }

  export interface ServerMessageTypeMap {
    NONE: 0;
    UPDATETICK: 1;
    VERIFYUUID: 2;
    VERIFIEDUUID: 3;
  }

  export const ServerMessageType: ServerMessageTypeMap;

  export enum MsgdataCase {
    MSGDATA_NOT_SET = 0,
    UPDATETICK = 2,
    VERIFYUUID = 100,
    VERIFIEDUUID = 101,
  }
}

