
//
// SFML - Simple and Fast Multimedia Library
// Copyright (C) 2007-2018 Laurent Gomila (laurent@sfml-dev.org)
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it freely,
// subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented;
//    you must not claim that you wrote the original software.
//    If you use this software in a product, an acknowledgment
//    in the product documentation would be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such,
//    and must not be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

// Headers

#include "System/Vector3.h"
#include <SFML/Window/Sensor.hpp>
#include <cstddef>

extern "C" bool sfSensor_isAvailable(sf::Sensor::Type sensor) {
    return sf::Sensor::isAvailable(sensor);
}

extern "C" void sfSensor_setEnabled(sf::Sensor::Type sensor, bool enabled) {
    sf::Sensor::setEnabled(sensor, enabled);
}

extern "C" sfVector3f sfSensor_getValue(sf::Sensor::Type sensor) {
    sf::Vector3f sfmlValue = sf::Sensor::getValue(sensor);

    sfVector3f value = {sfmlValue.x, sfmlValue.y, sfmlValue.z};
    return value;
}
