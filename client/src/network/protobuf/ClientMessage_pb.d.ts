// package: 
// file: ClientMessage.proto

import * as jspb from "google-protobuf";

export class ClientMessage extends jspb.Message {
  hasAck(): boolean;
  clearAck(): void;
  getAck(): ClientMessage.Ack | undefined;
  setAck(value?: ClientMessage.Ack): void;

  hasCommand(): boolean;
  clearCommand(): void;
  getCommand(): Command | undefined;
  setCommand(value?: Command): void;

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
    ack?: ClientMessage.Ack.AsObject,
    command?: Command.AsObject,
    veryfiyrtc?: ClientMessage.VerifyRtc.AsObject,
  }

  export class Ack extends jspb.Message {
    getNewbaseline(): number;
    setNewbaseline(value: number): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Ack.AsObject;
    static toObject(includeInstance: boolean, msg: Ack): Ack.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Ack, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Ack;
    static deserializeBinaryFromReader(message: Ack, reader: jspb.BinaryReader): Ack;
  }

  export namespace Ack {
    export type AsObject = {
      newbaseline: number,
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

  export enum MsgdataCase {
    MSGDATA_NOT_SET = 0,
    ACK = 1,
    COMMAND = 2,
    VERYFIYRTC = 100,
  }
}

export class Command extends jspb.Message {
  hasMovecommand(): boolean;
  clearMovecommand(): void;
  getMovecommand(): Command.MoveCommand | undefined;
  setMovecommand(value?: Command.MoveCommand): void;

  hasMovedelta(): boolean;
  clearMovedelta(): void;
  getMovedelta(): Command.MoveDelta | undefined;
  setMovedelta(value?: Command.MoveDelta): void;

  hasAttack(): boolean;
  clearAttack(): void;
  getAttack(): Command.Attack | undefined;
  setAttack(value?: Command.Attack): void;

  hasAbility(): boolean;
  clearAbility(): void;
  getAbility(): Ability | undefined;
  setAbility(value?: Ability): void;

  getCommandCase(): Command.CommandCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Command.AsObject;
  static toObject(includeInstance: boolean, msg: Command): Command.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Command, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Command;
  static deserializeBinaryFromReader(message: Command, reader: jspb.BinaryReader): Command;
}

export namespace Command {
  export type AsObject = {
    movecommand?: Command.MoveCommand.AsObject,
    movedelta?: Command.MoveDelta.AsObject,
    attack?: Command.Attack.AsObject,
    ability?: Ability.AsObject,
  }

  export class MoveCommand extends jspb.Message {
    getX(): number;
    setX(value: number): void;

    getY(): number;
    setY(value: number): void;

    getIsattack(): boolean;
    setIsattack(value: boolean): void;

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
      isattack: boolean,
    }
  }

  export class MoveDelta extends jspb.Message {
    getX(): number;
    setX(value: number): void;

    getY(): number;
    setY(value: number): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): MoveDelta.AsObject;
    static toObject(includeInstance: boolean, msg: MoveDelta): MoveDelta.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: MoveDelta, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): MoveDelta;
    static deserializeBinaryFromReader(message: MoveDelta, reader: jspb.BinaryReader): MoveDelta;
  }

  export namespace MoveDelta {
    export type AsObject = {
      x: number,
      y: number,
    }
  }

  export class Attack extends jspb.Message {
    getTarget(): number;
    setTarget(value: number): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Attack.AsObject;
    static toObject(includeInstance: boolean, msg: Attack): Attack.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Attack, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Attack;
    static deserializeBinaryFromReader(message: Attack, reader: jspb.BinaryReader): Attack;
  }

  export namespace Attack {
    export type AsObject = {
      target: number,
    }
  }

  export enum CommandCase {
    COMMAND_NOT_SET = 0,
    MOVECOMMAND = 1,
    MOVEDELTA = 2,
    ATTACK = 3,
    ABILITY = 4,
  }
}

export class Ability extends jspb.Message {
  getAbilityid(): number;
  setAbilityid(value: number): void;

  hasTargetedabilitydata(): boolean;
  clearTargetedabilitydata(): void;
  getTargetedabilitydata(): Ability.TargetedAbilityData | undefined;
  setTargetedabilitydata(value?: Ability.TargetedAbilityData): void;

  hasAimedabilitydata(): boolean;
  clearAimedabilitydata(): void;
  getAimedabilitydata(): Ability.AimedAbilityData | undefined;
  setAimedabilitydata(value?: Ability.AimedAbilityData): void;

  getAbilitydataCase(): Ability.AbilitydataCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Ability.AsObject;
  static toObject(includeInstance: boolean, msg: Ability): Ability.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Ability, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Ability;
  static deserializeBinaryFromReader(message: Ability, reader: jspb.BinaryReader): Ability;
}

export namespace Ability {
  export type AsObject = {
    abilityid: number,
    targetedabilitydata?: Ability.TargetedAbilityData.AsObject,
    aimedabilitydata?: Ability.AimedAbilityData.AsObject,
  }

  export class TargetedAbilityData extends jspb.Message {
    getTarget(): number;
    setTarget(value: number): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): TargetedAbilityData.AsObject;
    static toObject(includeInstance: boolean, msg: TargetedAbilityData): TargetedAbilityData.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: TargetedAbilityData, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): TargetedAbilityData;
    static deserializeBinaryFromReader(message: TargetedAbilityData, reader: jspb.BinaryReader): TargetedAbilityData;
  }

  export namespace TargetedAbilityData {
    export type AsObject = {
      target: number,
    }
  }

  export class AimedAbilityData extends jspb.Message {
    getX(): number;
    setX(value: number): void;

    getY(): number;
    setY(value: number): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AimedAbilityData.AsObject;
    static toObject(includeInstance: boolean, msg: AimedAbilityData): AimedAbilityData.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: AimedAbilityData, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AimedAbilityData;
    static deserializeBinaryFromReader(message: AimedAbilityData, reader: jspb.BinaryReader): AimedAbilityData;
  }

  export namespace AimedAbilityData {
    export type AsObject = {
      x: number,
      y: number,
    }
  }

  export interface AbilityDataTypeMap {
    SIMPLE: 0;
    TARGETED: 1;
    AIMED: 2;
  }

  export const AbilityDataType: AbilityDataTypeMap;

  export enum AbilitydataCase {
    ABILITYDATA_NOT_SET = 0,
    TARGETEDABILITYDATA = 2,
    AIMEDABILITYDATA = 3,
  }
}

