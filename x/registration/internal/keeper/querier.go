package keeper

import (
	"encoding/hex"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"

	abci "github.com/tendermint/tendermint/abci/types"
)

const (
	QueryEncryptedSeed = "seed"
	QueryMasterKey     = "master-key"
)

// controls error output on querier - set true when testing/debugging
const debug = false

// NewQuerier creates a new querier
func NewQuerier(keeper Keeper) sdk.Querier {
	return func(ctx sdk.Context, path []string, req abci.RequestQuery) ([]byte, error) {
		switch path[0] {
		case QueryEncryptedSeed:
			return queryEncryptedSeed(ctx, path[1], req, keeper)
		case QueryMasterKey:
			return queryMasterKey(ctx, req, keeper)
		default:
			return nil, sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, "unknown data query endpoint")
		}
	}
}

func queryMasterKey(ctx sdk.Context, req abci.RequestQuery, keeper Keeper) ([]byte, error) {
	seed := keeper.GetMasterPublicKey(ctx)
	if seed == nil {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnknownAddress, "Chain has not been initialized yet")
	}

	return *seed, nil
}

func queryEncryptedSeed(ctx sdk.Context, pubkey string, req abci.RequestQuery, keeper Keeper) ([]byte, error) {
	pubkeyBytes, err := hex.DecodeString(pubkey)
	if err != nil {
		return nil, err
	}

	seed := keeper.getRegistrationInfo(ctx, pubkeyBytes).EncryptedSeed
	if seed == nil {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnknownAddress, "Node has not been authenticated yet")
	}

	return seed, nil
}
