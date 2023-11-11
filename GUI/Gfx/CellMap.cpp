#include "CellMap.hpp"

CellMap::CellMap(sf::RenderWindow *window, Data *data, tool *Tool, std::map<uint8_t, sf::Color> *colors, Network *network) : _data(data), _window(window), _tool(Tool), _colors(colors), _network(network)
{
    _frame.setFillColor(sf::Color::Transparent);
    _frame.setOutlineColor(sf::Color::White);
    _frame.setOutlineThickness(2);

    _rect.setFillColor(sf::Color::Transparent);
    _rect.setOutlineColor(sf::Color::White);
    _rect.setOutlineThickness(1);
}

CellMap::~CellMap()
{
}

void CellMap::draw()
{
    _window->setView(_window->getDefaultView());
    float d;
    if (_data->getWidth() / (_window->getSize().x * 0.8) > _data->getHeight() / (float(_window->getSize().y - 120))) {
        d = (_window->getSize().x * 0.8) / _data->getWidth();
    } else {
        d = (float(_window->getSize().y - 120)) / _data->getHeight();
    }
    sf::Vector2f size = _window->mapPixelToCoords(sf::Vector2i(_data->getWidth(), _data->getHeight())) * d;
    sf::Vector2f origin = _window->mapPixelToCoords(sf::Vector2i(_data->getWidth(), _data->getHeight())) * d / 2.f;
    sf::Vector2f pos = _window->mapPixelToCoords((sf::Vector2i(_window->getSize()) - sf::Vector2i(0, 120)) / 2);

    _frame.setSize(size);
    _frame.setOrigin(origin);
    _frame.setOrigin(_frame.getOrigin());
    _frame.setPosition(pos + sf::Vector2f(0, 10));
    _rect.setSize(_window->mapPixelToCoords(sf::Vector2i(1, 1)) * d);
    _rect.setOrigin(sf::Vector2f(0, 0));


    sf::Vector2f startPos = pos - origin + sf::Vector2f(0, 10);

    for (uint32_t y = 0; y < _data->getHeight(); y++) {
        for (uint32_t x = 0; x < _data->getWidth(); x++) {
            if (_data->getCell(x, y) == 0) continue;
            _rect.setPosition(startPos + sf::Vector2f(x * _rect.getSize().x, y * _rect.getSize().y));
            _rect.setFillColor(_colors->at(_data->getCell(x, y)));
            _window->draw(_rect);
        }
    }
    _window->draw(_frame);
}

void CellMap::event(sf::Event *event)
{
    if (event->type == sf::Event::MouseButtonPressed) {
        if (event->mouseButton.button == sf::Mouse::Left) {
            float d;
            if (_data->getWidth() / (_window->getSize().x * 0.8) > _data->getHeight() / (float(_window->getSize().y - 120))) {
                d = (_window->getSize().x * 0.8) / _data->getWidth();
            } else {
                d = (float(_window->getSize().y - 120)) / _data->getHeight();
            }
            sf::Vector2f mousePos = _window->mapPixelToCoords(sf::Vector2i(event->mouseButton.x, event->mouseButton.y));
            sf::Vector2f pos = _window->mapPixelToCoords((sf::Vector2i(_window->getSize()) - sf::Vector2i(0, 120)) / 2);
            sf::Vector2f origin = _window->mapPixelToCoords(sf::Vector2i(_data->getWidth(), _data->getHeight())) * d / 2.f;


            sf::Vector2f startPos = pos - origin + sf::Vector2f(0, 10);
            for (uint32_t y = 0; y < _data->getHeight(); y++) {
                for (uint32_t x = 0; x < _data->getWidth(); x++) {
                    sf::IntRect rect(sf::Vector2i(startPos) + sf::Vector2i(x * _rect.getSize().x, y * _rect.getSize().y), sf::Vector2i(_rect.getSize()));
                    if (rect.contains(sf::Vector2i(mousePos))) {
                        _network->sendCell(x, y, _tool->color);
                        std::cout << std::to_string(x) + " " + std::to_string(y) + " " + std::to_string(_tool->color) << std::endl;
                        return;
                    }
                }
            }
        }
    }
}
