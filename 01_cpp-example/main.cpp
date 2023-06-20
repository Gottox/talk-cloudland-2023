/**
 * @author      : Enno Boland (mail@eboland.de)
 * @file        : main
 * @created     : Monday Jun 19, 2023 17:04:26 CEST
 */

#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Actor {
  private:
    string name;
    int year_of_birth;

  public:
    Actor(string name, int year_of_birth) {
        this->name = name;
        this->year_of_birth = year_of_birth;
    }

    friend ostream &
    operator<<(ostream &os, const Actor &obj) {
        os << obj.name << " (" << obj.year_of_birth << ")";
        return os;
    }

    int
    get_year_of_birth() {
        return year_of_birth;
    }
};

int
main() {
    cout << "Incomplete Movie Database (IMDB)" << endl;
    cout << "-------------------" << endl;

    vector<Actor> actors;

    actors.push_back(Actor("Tilda Swinton", 1960));
    actors.push_back(Actor("Tom Hanks", 1956));
    actors.push_back(Actor("Helena Bonham Carter", 1966));
    actors.push_back(Actor("Sandra Bullock", 1964));
    actors.push_back(Actor("Leonardo DiCaprio", 1974));
    actors.push_back(Actor("Tom Hiddleston", 1981));
    actors.push_back(Actor("Scarlett Johansson", 1984));
    actors.push_back(Actor("Liam Neeson", 1952));
    actors.push_back(Actor("Johnny Depp", 1963));

    Actor &tilda_swinton = actors[0];
    for (Actor actor : actors) {
        cout << actor << endl;
    }
    cout << "-------------------" << endl;
    cout << "First actor: " << tilda_swinton << endl;
    cout << "-------------------" << endl;
    cout << "... Sorting ..." << endl;
    sort(actors.begin(), actors.end(), [](Actor &a, Actor &b) {
        return a.get_year_of_birth() < b.get_year_of_birth();
    });
    cout << "-------------------" << endl;

    for (Actor actor : actors) {
        cout << actor << endl;
    }

    return 0;
}
