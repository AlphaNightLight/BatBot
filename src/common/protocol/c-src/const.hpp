#define BUFFER_SIZE 205 //ALMENO MAX_MESSAGE_SIZE + 4
#define START_SYMBOL '#'
#define END_SYMBOL 'E'
#define MAGIC_NUMBER 20
#define OUT_BUFFER_SIZE 200 // almeno della dimensione del messaggio
//sarebbe meglio che start e end siano > di OUT_BUFFER_SIZE (minori collisioni)
//inoltre  OUT_BUFFER_SIZE ha da essere il più piccolo possibile (idealmente della dimensione del messaggio più lungo) 
