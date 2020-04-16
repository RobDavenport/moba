// package: 
// file: ServerMessage.proto

import * as jspb from "google-protobuf";

export class ServerMessage extends jspb.Message {
  hasUpdatetick(): boolean;
  clearUpdatetick(): void;
  getUpdatetick(): ServerMessage.UpdateTick | undefined;
  setUpdatetick(value?: ServerMessage.UpdateTick): void;

  hasSnapshot(): boolean;
  clearSnapshot(): void;
  getSnapshot(): ServerMessage.Snapshot | undefined;
  setSnapshot(value?: ServerMessage.Snapshot): void;

  hasEntitydestroyed(): boolean;
  clearEntitydestroyed(): void;
  getEntitydestroyed(): ServerMessage.EntityDestroyed | undefined;
  setEntitydestroyed(value?: ServerMessage.EntityDestroyed): void;

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
    updatetick?: ServerMessage.UpdateTick.AsObject,
    snapshot?: ServerMessage.Snapshot.AsObject,
    entitydestroyed?: ServerMessage.EntityDestroyed.AsObject,
    verifyuuid?: ServerMessage.VerifyUuid.AsObject,
    verifieduuid?: ServerMessage.VerifiedUuid.AsObject,
  }

  export class UpdateTick extends jspb.Message {
    getFrame(): number;
    setFrame(value: number): void;

    hasEntitydata(): boolean;
    clearEntitydata(): void;
    getEntitydata(): ServerMessage.EntityData | undefined;
    setEntitydata(value?: ServerMessage.EntityData): void;

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
      entitydata?: ServerMessage.EntityData.AsObject,
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

  export class EntityDestroyed extends jspb.Message {
    getFrame(): number;
    setFrame(value: number): void;

    getReplicationid(): number;
    setReplicationid(value: number): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): EntityDestroyed.AsObject;
    static toObject(includeInstance: boolean, msg: EntityDestroyed): EntityDestroyed.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: EntityDestroyed, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): EntityDestroyed;
    static deserializeBinaryFromReader(message: EntityDestroyed, reader: jspb.BinaryReader): EntityDestroyed;
  }

  export namespace EntityDestroyed {
    export type AsObject = {
      frame: number,
      replicationid: number,
    }
  }

  export class Snapshot extends jspb.Message {
    getFrame(): number;
    setFrame(value: number): void;

    getBaseline(): number;
    setBaseline(value: number): void;

    clearEntitydataList(): void;
    getEntitydataList(): Array<ServerMessage.EntityData>;
    setEntitydataList(value: Array<ServerMessage.EntityData>): void;
    addEntitydata(value?: ServerMessage.EntityData, index?: number): ServerMessage.EntityData;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Snapshot.AsObject;
    static toObject(includeInstance: boolean, msg: Snapshot): Snapshot.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Snapshot, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Snapshot;
    static deserializeBinaryFromReader(message: Snapshot, reader: jspb.BinaryReader): Snapshot;
  }

  export namespace Snapshot {
    export type AsObject = {
      frame: number,
      baseline: number,
      entitydataList: Array<ServerMessage.EntityData.AsObject>,
    }
  }

  export class EntityData extends jspb.Message {
    getX(): number;
    setX(value: number): void;

    getY(): number;
    setY(value: number): void;

    getRotation(): number;
    setRotation(value: number): void;

    getReplicationid(): number;
    setReplicationid(value: number): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): EntityData.AsObject;
    static toObject(includeInstance: boolean, msg: EntityData): EntityData.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: EntityData, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): EntityData;
    static deserializeBinaryFromReader(message: EntityData, reader: jspb.BinaryReader): EntityData;
  }

  export namespace EntityData {
    export type AsObject = {
      x: number,
      y: number,
      rotation: number,
      replicationid: number,
    }
  }

  export enum MsgdataCase {
    MSGDATA_NOT_SET = 0,
    UPDATETICK = 1,
    SNAPSHOT = 2,
    ENTITYDESTROYED = 5,
    VERIFYUUID = 25,
    VERIFIEDUUID = 26,
  }
}

