// package: 
// file: ClientMessage.proto

import * as jspb from "google-protobuf";

export class ClientMessage extends jspb.Message {
  getMsgtype(): ClientMessage.ClientMessageTypeMap[keyof ClientMessage.ClientMessageTypeMap];
  setMsgtype(value: ClientMessage.ClientMessageTypeMap[keyof ClientMessage.ClientMessageTypeMap]): void;

  hasMovecommand(): boolean;
  clearMovecommand(): void;
  getMovecommand(): ClientMessage.MoveCommand | undefined;
  setMovecommand(value?: ClientMessage.MoveCommand): void;

  hasVeryfiyrtc(): boolean;
  clearVeryfiyrtc(): void;
  getVeryfiyrtc(): ClientMessage.VerifyRtc | undefined;
  setVeryfiyrtc(value?: ClientMessage.VerifyRtc): void;

  getMsgdataCase(): ClientMessage.MsgdataCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ClientMessage.AsObject;
  static toObject(includeInstance: boolean, msg: ClientMessage): ClientMessage.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ClientMessage, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ClientMessage;
  static deserializeBinaryFromReader(message: ClientMessage, reader: jspb.BinaryReader): ClientMessage;
}

export namespace ClientMessage {
  export type AsObject = {
    msgtype: ClientMessage.ClientMessageTypeMap[keyof ClientMessage.ClientMessageTypeMap],
    movecommand?: ClientMessage.MoveCommand.AsObject,
    veryfiyrtc?: ClientMessage.VerifyRtc.AsObject,
  }

  export class MoveCommand extends jspb.Message {
    getX(): number;
    setX(value: number): void;

    getY(): number;
    setY(value: number): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): MoveCommand.AsObject;
    static toObject(includeInstance: boolean, msg: MoveCommand): MoveCommand.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: MoveCommand, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): MoveCommand;
    static deserializeBinaryFromReader(message: MoveCommand, reader: jspb.BinaryReader): MoveCommand;
  }

  export namespace MoveCommand {
    export type AsObject = {
      x: number,
      y: number,
    }
  }

  export class VerifyRtc extends jspb.Message {
    getUuid(): string;
    setUuid(value: string): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): VerifyRtc.AsObject;
    static toObject(includeInstance: boolean, msg: VerifyRtc): VerifyRtc.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: VerifyRtc, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): VerifyRtc;
    static deserializeBinaryFromReader(message: VerifyRtc, reader: jspb.BinaryReader): VerifyRtc;
  }

  export namespace VerifyRtc {
    export type AsObject = {
      uuid: string,
    }
  }

  export interface ClientMessageTypeMap {
    NONE: 0;
    MOVE: 1;
    VERIFYRTC: 100;
  }

  export const ClientMessageType: ClientMessageTypeMap;

  export enum MsgdataCase {
    MSGDATA_NOT_SET = 0,
    MOVECOMMAND = 2,
    VERYFIYRTC = 100,
  }
}

