package com.sashaflake.infrastructure.persistence

import com.sashaflake.domain.city.City
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.sql.Connection
import java.sql.Statement

class CityRepository(private val connection: Connection) {

    companion object {
        private const val CREATE_TABLE =
            "CREATE TABLE IF NOT EXISTS cities (id SERIAL PRIMARY KEY, name VARCHAR(255), population INT);"
        private const val SELECT_BY_ID = "SELECT name, population FROM cities WHERE id = ?"
        private const val INSERT = "INSERT INTO cities (name, population) VALUES (?, ?)"
        private const val UPDATE = "UPDATE cities SET name = ?, population = ? WHERE id = ?"
        private const val DELETE = "DELETE FROM cities WHERE id = ?"
    }

    init {
        connection.createStatement().executeUpdate(CREATE_TABLE)
    }

    suspend fun create(city: City): Int = withContext(Dispatchers.IO) {
        val stmt = connection.prepareStatement(INSERT, Statement.RETURN_GENERATED_KEYS)
        stmt.setString(1, city.name)
        stmt.setInt(2, city.population)
        stmt.executeUpdate()
        val keys = stmt.generatedKeys
        if (keys.next()) keys.getInt(1)
        else throw IllegalStateException("Failed to retrieve generated city id")
    }

    suspend fun read(id: Int): City = withContext(Dispatchers.IO) {
        val stmt = connection.prepareStatement(SELECT_BY_ID)
        stmt.setInt(1, id)
        val rs = stmt.executeQuery()
        if (rs.next()) City(rs.getString("name"), rs.getInt("population"))
        else throw NoSuchElementException("City not found: $id")
    }

    suspend fun update(id: Int, city: City) = withContext(Dispatchers.IO) {
        val stmt = connection.prepareStatement(UPDATE)
        stmt.setString(1, city.name)
        stmt.setInt(2, city.population)
        stmt.setInt(3, id)
        stmt.executeUpdate()
    }

    suspend fun delete(id: Int) = withContext(Dispatchers.IO) {
        val stmt = connection.prepareStatement(DELETE)
        stmt.setInt(1, id)
        stmt.executeUpdate()
    }
}
