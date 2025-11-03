#pragma once

#include "component_metadata.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::ComponentMetadata*, subscribe_metadata_available, mavsdk::ComponentMetadata::MetadataAvailableCallback)
}