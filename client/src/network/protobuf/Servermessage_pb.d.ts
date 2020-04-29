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

    hasBaseline(): boolean;
    clearBaseline(): void;
    getBaseline(): number;
    setBaseline(value: number): void;

    clearEntitydataList(): void;
    getEntitydataList(): Array<ServerMessage.EntityData>;
    setEntitydataList(value: Array<ServerMessage.EntityData>): void;
    addEntitydata(value?: ServerMessage.EntityData, index?: number): ServerMessage.EntityData;

    getOptionBaselineCase(): Snapshot.OptionBaselineCase;
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

    export enum OptionBaselineCase {
      OPTION_BASELINE_NOT_SET = 0,
      BASELINE = 2,
    }
  }

  export class EntityData extends jspb.Message {
    getReplicationid(): number;
    setReplicationid(value: number): void;

    hasX(): boolean;
    clearX(): void;
    getX(): number;
    setX(value: number): void;

    hasY(): boolean;
    clearY(): void;
    getY(): number;
    setY(value: number): void;

    hasRotation(): boolean;
    clearRotation(): void;
    getRotation(): number;
    setRotation(value: number): void;

    hasHealth(): boolean;
    clearHealth(): void;
    getHealth(): number;
    setHealth(value: number): void;

    hasEnergy(): boolean;
    clearEnergy(): void;
    getEnergy(): number;
    setEnergy(value: number): void;

    hasEntitytypedata(): boolean;
    clearEntitytypedata(): void;
    getEntitytypedata(): ServerMessage.EntityTypeData | undefined;
    setEntitytypedata(value?: ServerMessage.EntityTypeData): void;

    getOptionXCase(): EntityData.OptionXCase;
    getOptionYCase(): EntityData.OptionYCase;
    getOptionRotationCase(): EntityData.OptionRotationCase;
    getOptionHealthCase(): EntityData.OptionHealthCase;
    getOptionEnergyCase(): EntityData.OptionEnergyCase;
    getOptionEntityTypeDataCase(): EntityData.OptionEntityTypeDataCase;
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
      replicationid: number,
      x: number,
      y: number,
      rotation: number,
      health: number,
      energy: number,
      entitytypedata?: ServerMessage.EntityTypeData.AsObject,
    }

    export enum OptionXCase {
      OPTION_X_NOT_SET = 0,
      X = 2,
    }

    export enum OptionYCase {
      OPTION_Y_NOT_SET = 0,
      Y = 3,
    }

    export enum OptionRotationCase {
      OPTION_ROTATION_NOT_SET = 0,
      ROTATION = 4,
    }

    export enum OptionHealthCase {
      OPTION_HEALTH_NOT_SET = 0,
      HEALTH = 5,
    }

    export enum OptionEnergyCase {
      OPTION_ENERGY_NOT_SET = 0,
      ENERGY = 6,
    }

    export enum OptionEntityTypeDataCase {
      OPTION_ENTITY_TYPE_DATA_NOT_SET = 0,
      ENTITYTYPEDATA = 7,
    }
  }

  export class EntityTypeData extends jspb.Message {
    hasCharacterdata(): boolean;
    clearCharacterdata(): void;
    getCharacterdata(): ServerMessage.EntityTypeData.CharacterData | undefined;
    setCharacterdata(value?: ServerMessage.EntityTypeData.CharacterData): void;

    hasMiniondata(): boolean;
    clearMiniondata(): void;
    getMiniondata(): ServerMessage.EntityTypeData.MinionData | undefined;
    setMiniondata(value?: ServerMessage.EntityTypeData.MinionData): void;

    hasTowerdata(): boolean;
    clearTowerdata(): void;
    getTowerdata(): ServerMessage.EntityTypeData.TowerData | undefined;
    setTowerdata(value?: ServerMessage.EntityTypeData.TowerData): void;

    hasCoredata(): boolean;
    clearCoredata(): void;
    getCoredata(): ServerMessage.EntityTypeData.CoreData | undefined;
    setCoredata(value?: ServerMessage.EntityTypeData.CoreData): void;

    getEntitydataCase(): EntityTypeData.EntitydataCase;
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): EntityTypeData.AsObject;
    static toObject(includeInstance: boolean, msg: EntityTypeData): EntityTypeData.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: EntityTypeData, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): EntityTypeData;
    static deserializeBinaryFromReader(message: EntityTypeData, reader: jspb.BinaryReader): EntityTypeData;
  }

  export namespace EntityTypeData {
    export type AsObject = {
      characterdata?: ServerMessage.EntityTypeData.CharacterData.AsObject,
      miniondata?: ServerMessage.EntityTypeData.MinionData.AsObject,
      towerdata?: ServerMessage.EntityTypeData.TowerData.AsObject,
      coredata?: ServerMessage.EntityTypeData.CoreData.AsObject,
    }

    export class CharacterData extends jspb.Message {
      hasMaxhealth(): boolean;
      clearMaxhealth(): void;
      getMaxhealth(): number;
      setMaxhealth(value: number): void;

      hasMaxenergy(): boolean;
      clearMaxenergy(): void;
      getMaxenergy(): number;
      setMaxenergy(value: number): void;

      hasEnergytype(): boolean;
      clearEnergytype(): void;
      getEnergytype(): ServerMessage.EntityTypeData.CharacterData.EnergyTypeMap[keyof ServerMessage.EntityTypeData.CharacterData.EnergyTypeMap];
      setEnergytype(value: ServerMessage.EntityTypeData.CharacterData.EnergyTypeMap[keyof ServerMessage.EntityTypeData.CharacterData.EnergyTypeMap]): void;

      getOptionMaxHealthCase(): CharacterData.OptionMaxHealthCase;
      getOptionMaxEnergyCase(): CharacterData.OptionMaxEnergyCase;
      getOptionEnergyTypeCase(): CharacterData.OptionEnergyTypeCase;
      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): CharacterData.AsObject;
      static toObject(includeInstance: boolean, msg: CharacterData): CharacterData.AsObject;
      static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
      static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
      static serializeBinaryToWriter(message: CharacterData, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): CharacterData;
      static deserializeBinaryFromReader(message: CharacterData, reader: jspb.BinaryReader): CharacterData;
    }

    export namespace CharacterData {
      export type AsObject = {
        maxhealth: number,
        maxenergy: number,
        energytype: ServerMessage.EntityTypeData.CharacterData.EnergyTypeMap[keyof ServerMessage.EntityTypeData.CharacterData.EnergyTypeMap],
      }

      export interface EnergyTypeMap {
        NONE: 0;
        MANA: 1;
        ENERGY: 2;
      }

      export const EnergyType: EnergyTypeMap;

      export enum OptionMaxHealthCase {
        OPTION_MAX_HEALTH_NOT_SET = 0,
        MAXHEALTH = 1,
      }

      export enum OptionMaxEnergyCase {
        OPTION_MAX_ENERGY_NOT_SET = 0,
        MAXENERGY = 2,
      }

      export enum OptionEnergyTypeCase {
        OPTION_ENERGY_TYPE_NOT_SET = 0,
        ENERGYTYPE = 3,
      }
    }

    export class MinionData extends jspb.Message {
      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): MinionData.AsObject;
      static toObject(includeInstance: boolean, msg: MinionData): MinionData.AsObject;
      static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
      static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
      static serializeBinaryToWriter(message: MinionData, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): MinionData;
      static deserializeBinaryFromReader(message: MinionData, reader: jspb.BinaryReader): MinionData;
    }

    export namespace MinionData {
      export type AsObject = {
      }
    }

    export class TowerData extends jspb.Message {
      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): TowerData.AsObject;
      static toObject(includeInstance: boolean, msg: TowerData): TowerData.AsObject;
      static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
      static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
      static serializeBinaryToWriter(message: TowerData, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): TowerData;
      static deserializeBinaryFromReader(message: TowerData, reader: jspb.BinaryReader): TowerData;
    }

    export namespace TowerData {
      export type AsObject = {
      }
    }

    export class CoreData extends jspb.Message {
      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): CoreData.AsObject;
      static toObject(includeInstance: boolean, msg: CoreData): CoreData.AsObject;
      static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
      static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
      static serializeBinaryToWriter(message: CoreData, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): CoreData;
      static deserializeBinaryFromReader(message: CoreData, reader: jspb.BinaryReader): CoreData;
    }

    export namespace CoreData {
      export type AsObject = {
      }
    }

    export enum EntitydataCase {
      ENTITYDATA_NOT_SET = 0,
      CHARACTERDATA = 1,
      MINIONDATA = 2,
      TOWERDATA = 3,
      COREDATA = 4,
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

